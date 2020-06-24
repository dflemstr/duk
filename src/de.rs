use serde::de::DeserializeSeed;
use serde::de::Deserializer;
use serde::de::EnumAccess;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;

#[derive(Clone, Copy, Debug)]
struct DukDe {
    ctx: *mut duk_sys::duk_context,
    idx: i32,
}

#[derive(Debug)]
pub struct Error(String);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for Error {}
impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error(format!("{}", msg))
    }
}

impl<'de> Deserializer<'de> for DukDe {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            match duk_sys::duk_get_type(self.ctx, self.idx) as u32 {
                duk_sys::DUK_TYPE_UNDEFINED => self.deserialize_unit(visitor),
                duk_sys::DUK_TYPE_NULL => self.deserialize_unit(visitor),
                duk_sys::DUK_TYPE_BOOLEAN => self.deserialize_bool(visitor),
                duk_sys::DUK_TYPE_NUMBER => self.deserialize_f64(visitor),
                duk_sys::DUK_TYPE_STRING => self.deserialize_str(visitor),
                duk_sys::DUK_TYPE_OBJECT => {
                    if duk_sys::duk_is_array(self.ctx, self.idx) == 1 {
                        self.deserialize_seq(visitor)
                    } else {
                        self.deserialize_map(visitor)
                    }
                }
                duk_sys::DUK_TYPE_BUFFER => self.deserialize_bytes(visitor),
                duk_sys::DUK_TYPE_POINTER => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("pointer"),
                    &"value",
                )),
                duk_sys::DUK_TYPE_LIGHTFUNC => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("lightfunc"),
                    &"value",
                )),
                _ => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("unknown"),
                    &"value",
                )),
            }
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_bool(duk_sys::duk_get_boolean(self.ctx, self.idx) != 0) }
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_i8(duk_sys::duk_get_number(self.ctx, self.idx) as i8) }
    }
    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_i16(duk_sys::duk_get_number(self.ctx, self.idx) as i16) }
    }
    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_i32(duk_sys::duk_get_number(self.ctx, self.idx) as i32) }
    }
    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_i64(duk_sys::duk_get_number(self.ctx, self.idx) as i64) }
    }
    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_u8(duk_sys::duk_get_number(self.ctx, self.idx) as u8) }
    }
    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_u16(duk_sys::duk_get_number(self.ctx, self.idx) as u16) }
    }
    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_u32(duk_sys::duk_get_number(self.ctx, self.idx) as u32) }
    }
    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_u64(duk_sys::duk_get_number(self.ctx, self.idx) as u64) }
    }
    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_f32(duk_sys::duk_get_number(self.ctx, self.idx) as f32) }
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_f64(duk_sys::duk_get_number(self.ctx, self.idx) as f64) }
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            if let Some(c) = crate::get_str(self.ctx, self.idx).chars().next() {
                visitor.visit_char(c)
            } else {
                visitor.visit_none()
            }
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_str(crate::get_str(self.ctx, self.idx)) }
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe { visitor.visit_string(crate::get_string(self.ctx, self.idx)) }
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            let mut size = 0;
            let size_ptr = &mut size;
            let res = duk_sys::duk_get_buffer(self.ctx, self.idx, size_ptr);
            let res = std::slice::from_raw_parts(res.cast(), size);

            visitor.visit_bytes(res)
        }
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            let mut size = 0;
            let size_ptr = &mut size;
            let res = duk_sys::duk_get_buffer(self.ctx, self.idx, size_ptr);
            let res = std::slice::from_raw_parts(res.cast(), size);

            visitor.visit_byte_buf(res.to_vec())
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            match duk_sys::duk_get_type(self.ctx, self.idx) as u32 {
                duk_sys::DUK_TYPE_NULL | duk_sys::DUK_TYPE_UNDEFINED => visitor.visit_none(),
                _ => visitor.visit_some(self),
            }
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_unit()
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_unit() // TODO: is this right?
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            visitor.visit_seq(SeqAccessor {
                de: self,
                len: duk_sys::duk_get_length(self.ctx, self.idx),
                pos: 0,
            })
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Error> {
        unsafe {
            let real_len = duk_sys::duk_get_length(self.ctx, self.idx);
            if real_len == len {
                visitor.visit_seq(SeqAccessor {
                    de: self,
                    len,
                    pos: 0,
                })
            } else {
                Err(serde::de::Error::invalid_length(
                    len,
                    &format!("{}", real_len).as_str(),
                ))
            }
        }
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        struct MapAccessor(DukDe);
        impl<'de> MapAccess<'de> for MapAccessor {
            type Error = Error;

            fn next_key_seed<K: DeserializeSeed<'de>>(
                &mut self,
                seed: K,
            ) -> Result<Option<K::Value>, Self::Error> {
                unsafe {
                    if duk_sys::duk_next(self.0.ctx, -1, 1) == 1 {
                        seed.deserialize(DukDe {
                            ctx: self.0.ctx,
                            idx: -2,
                        })
                        .map(Some)
                    } else {
                        Ok(None)
                    }
                }
            }

            fn next_value_seed<V: DeserializeSeed<'de>>(
                &mut self,
                seed: V,
            ) -> Result<V::Value, Self::Error> {
                unsafe {
                    let res = seed.deserialize(DukDe {
                        ctx: self.0.ctx,
                        idx: -1,
                    });
                    duk_sys::duk_pop_2(self.0.ctx);
                    res
                }
            }
        }

        unsafe {
            let start = duk_sys::duk_get_top(self.ctx);

            duk_sys::duk_enum(self.ctx, self.idx, duk_sys::DUK_ENUM_OWN_PROPERTIES_ONLY);

            let res = visitor.visit_map(MapAccessor(self));

            duk_sys::duk_pop_n(self.ctx, duk_sys::duk_get_top(self.ctx) - start);
            // clean up stack

            res
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        struct MapAccessor {
            de: DukDe,
            keys: &'static [&'static str],
            pos: usize,
        }
        impl<'de> MapAccess<'de> for MapAccessor {
            type Error = Error;

            fn next_key_seed<K: DeserializeSeed<'de>>(
                &mut self,
                seed: K,
            ) -> Result<Option<K::Value>, Self::Error> {
                unsafe {
                    while self.pos < self.keys.len()
                        && duk_sys::duk_has_prop_lstring(
                            self.de.ctx,
                            self.de.idx,
                            self.keys[self.pos].as_ptr().cast(),
                            self.keys[self.pos].len(),
                        ) == 0
                    {
                        self.pos += 1;
                    }
                    if self.pos >= self.keys.len() {
                        return Ok(None);
                    }
                    duk_sys::duk_push_lstring(
                        self.de.ctx,
                        self.keys[self.pos].as_ptr().cast(),
                        self.keys[self.pos].len(),
                    );
                    self.pos += 1;
                    seed.deserialize(DukDe {
                        ctx: self.de.ctx,
                        idx: duk_sys::duk_get_top_index(self.de.ctx),
                    })
                    .map(Some)
                }
            }

            fn next_value_seed<V: DeserializeSeed<'de>>(
                &mut self,
                seed: V,
            ) -> Result<V::Value, Self::Error> {
                unsafe {
                    duk_sys::duk_get_prop(self.de.ctx, self.de.idx);
                    let res = seed.deserialize(DukDe {
                        ctx: self.de.ctx,
                        idx: duk_sys::duk_get_top_index(self.de.ctx),
                    });
                    duk_sys::duk_pop(self.de.ctx);
                    res
                }
            }
        }

        unsafe {
            let start = duk_sys::duk_get_top(self.ctx);

            duk_sys::duk_enum(self.ctx, self.idx, duk_sys::DUK_ENUM_OWN_PROPERTIES_ONLY);

            let res = visitor.visit_map(MapAccessor {
                de: self,
                keys: fields,
                pos: 0,
            });

            duk_sys::duk_pop_n(self.ctx, duk_sys::duk_get_top(self.ctx) - start); // clean up stack

            res
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        struct EnumAccessor(DukDe);
        impl<'de> EnumAccess<'de> for EnumAccessor {
            type Error = Error;
            type Variant = VariantAccessor;

            fn variant_seed<V: DeserializeSeed<'de>>(
                self,
                seed: V,
            ) -> Result<(V::Value, Self::Variant), Self::Error> {
                unsafe {
                    duk_sys::duk_enum(
                        self.0.ctx,
                        self.0.idx,
                        duk_sys::DUK_ENUM_OWN_PROPERTIES_ONLY,
                    );

                    if duk_sys::duk_next(self.0.ctx, -1, 1) == 1 {
                        let res = seed
                            .deserialize(DukDe {
                                ctx: self.0.ctx,
                                idx: duk_sys::duk_get_top(self.0.ctx) - 2,
                            })
                            .map(|v| {
                                (
                                    v,
                                    VariantAccessor(DukDe {
                                        ctx: self.0.ctx,
                                        idx: duk_sys::duk_get_top(self.0.ctx) - 1,
                                    }),
                                )
                            });
                        res
                    } else {
                        Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Map,
                            &"string",
                        ))
                    }
                }
            }
        }
        struct VariantAccessor(DukDe);
        impl<'de> VariantAccess<'de> for VariantAccessor {
            type Error = Error;

            fn unit_variant(self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                unsafe {
                    seed.deserialize(DukDe {
                        ctx: self.0.ctx,
                        idx: duk_sys::duk_get_top_index(self.0.ctx),
                    })
                }
            }

            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.0.deserialize_tuple(len, visitor)
            }

            fn struct_variant<V>(
                self,
                fields: &'static [&'static str],
                visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.0.deserialize_struct("", fields, visitor)
            }
        }

        struct UnitEnumAccessor(DukDe);
        impl<'de> EnumAccess<'de> for UnitEnumAccessor {
            type Error = Error;
            type Variant = UnitVariantAccessor;

            fn variant_seed<V: DeserializeSeed<'de>>(
                self,
                seed: V,
            ) -> Result<(V::Value, Self::Variant), Self::Error> {
                seed.deserialize(self.0).map(|v| (v, UnitVariantAccessor))
            }
        }
        struct UnitVariantAccessor;
        impl<'de> VariantAccess<'de> for UnitVariantAccessor {
            type Error = Error;

            fn unit_variant(self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                let unexp = serde::de::Unexpected::UnitVariant;
                Err(serde::de::Error::invalid_type(unexp, &"newtype variant"))
            }

            fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                let unexp = serde::de::Unexpected::UnitVariant;
                Err(serde::de::Error::invalid_type(unexp, &"tuple variant"))
            }

            fn struct_variant<V>(
                self,
                _fields: &'static [&'static str],
                _visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                let unexp = serde::de::Unexpected::UnitVariant;
                Err(serde::de::Error::invalid_type(unexp, &"struct variant"))
            }
        }

        unsafe {
            let start = duk_sys::duk_get_top(self.ctx);
            let res =
                if duk_sys::duk_get_type(self.ctx, self.idx) as u32 == duk_sys::DUK_TYPE_OBJECT {
                    visitor.visit_enum(EnumAccessor(self))
                } else {
                    visitor.visit_enum(UnitEnumAccessor(self))
                };

            duk_sys::duk_pop_n(self.ctx, duk_sys::duk_get_top(self.ctx) - start); // clean up stack

            res
        }
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_any(visitor)
    }
}
struct SeqAccessor {
    de: DukDe,
    len: usize,
    pos: usize,
}
impl<'de> SeqAccess<'de> for SeqAccessor {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        unsafe {
            if self.pos < self.len {
                debug_assert!(
                    duk_sys::duk_get_prop_index(self.de.ctx, self.de.idx, self.pos as u32) == 1
                );
                self.pos += 1;
                let res = seed.deserialize(DukDe {
                    ctx: self.de.ctx,
                    idx: duk_sys::duk_get_top_index(self.de.ctx),
                });
                duk_sys::duk_pop(self.de.ctx);
                res.map(Some)
            } else {
                Ok(None)
            }
        }
    }
}

pub unsafe fn deserialize_from_stack<'de, T: serde::Deserialize<'de>>(
    ctx: *mut duk_sys::duk_context,
    index: i32,
) -> Result<T, Error> {
    let _guard = crate::StackRAII::new(ctx);
    let top = duk_sys::duk_get_top(ctx);
    let idx = if index >= top {
        return Err(serde::de::Error::custom("index out of bounds"));
    } else if index >= 0 {
        index
    } else {
        top + index
    };
    T::deserialize(DukDe { ctx, idx })
}
