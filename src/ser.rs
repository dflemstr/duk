use serde::ser::Serialize;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::ser::SerializeStructVariant;
use serde::ser::SerializeTuple;
use serde::ser::SerializeTupleStruct;
use serde::ser::SerializeTupleVariant;
use serde::ser::Serializer;

#[derive(Debug)]
pub struct Error(String);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for Error {}
impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error(format!("{}", msg))
    }
}

trait HasCtx {
    fn ctx(&self) -> *mut duk_sys::duk_context;
}

#[derive(Clone, Copy, Debug)]
struct DukSer {
    ctx: *mut duk_sys::duk_context,
}
impl HasCtx for DukSer {
    fn ctx(&self) -> *mut duk_sys::duk_context {
        self.ctx
    }
}

impl Serializer for DukSer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSer;
    type SerializeTuple = SeqSer;
    type SerializeTupleStruct = SeqSer;
    type SerializeTupleVariant = VariantSer<SeqSer>;
    type SerializeMap = MapSer;
    type SerializeStruct = MapSer;
    type SerializeStructVariant = VariantSer<MapSer>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        unsafe {
            duk_sys::duk_push_boolean(self.ctx, if v { 1 } else { 0 });
            Ok(())
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unsafe {
            duk_sys::duk_push_number(self.ctx, v);
            Ok(())
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unsafe {
            duk_sys::duk_push_lstring(self.ctx, [v].as_ptr().cast(), 1);
            Ok(())
        }
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        unsafe {
            duk_sys::duk_push_lstring(self.ctx, v.as_ptr().cast(), v.len());
            Ok(())
        }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unsafe {
            let ptr = duk_sys::duk_push_buffer(self.ctx, v.len(), 0);
            std::ptr::copy(v.as_ptr(), ptr.cast(), v.len());
            Ok(())
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error> {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unsafe {
            duk_sys::duk_push_null(self.ctx);
            Ok(())
        }
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unsafe {
            let idx = duk_sys::duk_push_object(self.ctx);
            value.serialize(self)?;
            duk_sys::duk_put_prop_lstring(self.ctx, idx, variant.as_ptr().cast(), variant.len());
            Ok(())
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unsafe {
            let idx = duk_sys::duk_push_array(self.ctx);

            Ok(SeqSer {
                ser: self,
                idx,
                pos: 0,
            })
        }
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unsafe {
            let obj_idx = duk_sys::duk_push_object(self.ctx);
            let arr_idx = duk_sys::duk_push_array(self.ctx);

            Ok(VariantSer {
                ser: SeqSer {
                    ser: self,
                    idx: arr_idx,
                    pos: 0,
                },
                variant,
                obj_idx,
            })
        }
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unsafe {
            let idx = duk_sys::duk_push_object(self.ctx);

            Ok(MapSer { ser: self, idx })
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unsafe {
            let obj_idx = duk_sys::duk_push_object(self.ctx);
            let inner_idx = duk_sys::duk_push_object(self.ctx);

            Ok(VariantSer {
                ser: MapSer {
                    ser: self,
                    idx: inner_idx,
                },
                variant,
                obj_idx,
            })
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SeqSer {
    ser: DukSer,
    idx: i32,
    pos: u32,
}
impl HasCtx for SeqSer {
    fn ctx(&self) -> *mut duk_sys::duk_context {
        self.ser.ctx()
    }
}
impl SerializeSeq for SeqSer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unsafe {
            value.serialize(self.ser)?;
            duk_sys::duk_put_prop_index(self.ser.ctx, self.idx, self.pos);
            self.pos += 1;

            Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl SerializeTuple for SeqSer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}
impl SerializeTupleStruct for SeqSer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

#[derive(Clone, Copy, Debug)]
struct VariantSer<T: HasCtx + Copy> {
    ser: T,
    variant: &'static str,
    obj_idx: i32,
}
impl<T: HasCtx + Copy> HasCtx for VariantSer<T> {
    fn ctx(&self) -> *mut duk_sys::duk_context {
        self.ser.ctx()
    }
}
impl<T: HasCtx + Copy> VariantSer<T> {
    unsafe fn var_end(self) -> Result<(), Error> {
        duk_sys::duk_put_prop_lstring(
            self.ctx(),
            self.obj_idx,
            self.variant.as_ptr().cast(),
            self.variant.len(),
        );
        Ok(())
    }
}
impl SerializeTupleVariant for VariantSer<SeqSer> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_field(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unsafe {
            let ctx = self.ctx();
            let arr_idx = self.ser.idx;
            SerializeTuple::end(self.ser)?;
            debug_assert!(duk_sys::duk_get_top_index(ctx) == arr_idx);
            self.var_end()
        }
    }
}
impl SerializeStructVariant for VariantSer<MapSer> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unsafe {
            SerializeStruct::end(self.ser)?;
            debug_assert!(duk_sys::duk_get_top_index(self.ctx()) == self.ser.idx);
            self.var_end()
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct MapSer {
    ser: DukSer,
    idx: i32,
}
impl HasCtx for MapSer {
    fn ctx(&self) -> *mut duk_sys::duk_context {
        self.ser.ctx()
    }
}
impl SerializeMap for MapSer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<Self::Ok, Self::Error> {
        key.serialize(self.ser)
    }

    fn serialize_value<T: Serialize + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unsafe {
            value.serialize(self.ser)?;
            duk_sys::duk_put_prop(self.ctx(), self.idx);
            Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl SerializeStruct for MapSer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unsafe {
            value.serialize(self.ser)?;
            duk_sys::duk_put_prop_lstring(self.ctx(), self.idx, key.as_ptr().cast(), key.len());

            Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub unsafe fn serialize_to_stack<T: Serialize + ?Sized>(
    ctx: *mut duk_sys::duk_context,
    value: &T,
) -> Result<i32, Error> {
    let mut guard = crate::StackRAII::new(ctx);

    value.serialize(DukSer { ctx })?;

    guard.push();

    Ok(guard.idx())
}
