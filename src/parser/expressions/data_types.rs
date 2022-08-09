use inkwell::{context::Context, types::{AnyTypeEnum, BasicTypeEnum, BasicMetadataTypeEnum, FunctionType}, values::{AnyValueEnum, BasicValueEnum}};

use crate::lexer::lexer::{Token, TokenType};


#[derive(Debug, Clone)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    Void,
    Custom(Vec<DataType>, bool),
    Unknown
}
impl DataType {
    pub fn parse(token: &Token) -> Result<DataType, String> {
        match token.token_type {
            TokenType::Identifier(ref s) => {
                match s.as_str() {
                    "i8" => Ok(DataType::I8),
                    "i16" => Ok(DataType::I16),
                    "i32" => Ok(DataType::I32),
                    "i64" => Ok(DataType::I64),
                    "u8" => Ok(DataType::U8),
                    "u16" => Ok(DataType::U16),
                    "u32" => Ok(DataType::U32),
                    "u64" => Ok(DataType::U64),
                    "f32" => Ok(DataType::F32),
                    "f64" => Ok(DataType::F64),
                    "bool" => Ok(DataType::Bool),
                    "string" => Ok(DataType::String),
                    "void" => Ok(DataType::Void),
                    _ => Ok(DataType::Custom(vec![], false)),
                }
            }
            _ => Err(format!("Expected type, found {:?}", token)),
        }
    }

    pub fn into_llvm_type<'a>(&self, context: &'a Context) -> AnyTypeEnum<'a> {
        match self {
            DataType::I8 => AnyTypeEnum::IntType(context.i8_type()),
            DataType::I16 => AnyTypeEnum::IntType(context.i16_type()),
            DataType::I32 => AnyTypeEnum::IntType(context.i32_type()),
            DataType::I64 => AnyTypeEnum::IntType(context.i64_type()),
            DataType::U8 => todo!("u8"),
            DataType::U16 => todo!("u16"),
            DataType::U32 => todo!("u32"),
            DataType::U64 => todo!("u64"),
            DataType::F32 => AnyTypeEnum::FloatType(context.f32_type()),
            DataType::F64 => AnyTypeEnum::FloatType(context.f64_type()),
            DataType::Bool => AnyTypeEnum::IntType(context.bool_type()),
            DataType::String => AnyTypeEnum::PointerType(context.i8_type().ptr_type(inkwell::AddressSpace::Global)),
            DataType::Void => AnyTypeEnum::VoidType(context.void_type()),
            DataType::Custom(dt, v) => {
                let mut types: Vec<BasicTypeEnum> = Vec::new();
                for dt in dt {
                    types.push(dt.into_basic_type(context));
                }
                let type_name = context.struct_type(&types, false);
                AnyTypeEnum::StructType(type_name)
            }
            DataType::Unknown => AnyTypeEnum::VoidType(context.void_type()), // TODO: Replace this with a proper error message
        }
    }

    pub fn into_basic_type<'a>(&self, context: &'a Context) -> BasicTypeEnum<'a> {
        match self {
            DataType::I8 => BasicTypeEnum::IntType(context.i8_type()),
            DataType::I16 => BasicTypeEnum::IntType(context.i16_type()),
            DataType::I32 => BasicTypeEnum::IntType(context.i32_type()),
            DataType::I64 => BasicTypeEnum::IntType(context.i64_type()),
            DataType::U8 => todo!("u8"),
            DataType::U16 => todo!("u16"),
            DataType::U32 => todo!("u32"),
            DataType::U64 => todo!("u64"),
            DataType::F32 => BasicTypeEnum::FloatType(context.f32_type()),
            DataType::F64 => BasicTypeEnum::FloatType(context.f64_type()),
            DataType::Bool => BasicTypeEnum::IntType(context.bool_type()),
            DataType::String => BasicTypeEnum::PointerType(context.i8_type().ptr_type(inkwell::AddressSpace::Global)),
            DataType::Custom(dt, v) => {
                let mut types: Vec<BasicTypeEnum> = Vec::new();
                for dt in dt {
                    types.push(dt.into_basic_type(context));
                }
                let type_name = context.struct_type(&types, false);
                BasicTypeEnum::StructType(type_name)
            }
            _ => panic!("Type {:?} cannot be converted into BasicType", self), // TODO: change this message to be more meaningful
        }
    }

    pub fn into_basic_metadata_type<'a>(&self, context: &'a Context) -> BasicMetadataTypeEnum<'a> {
        match self {
            DataType::I8 => BasicMetadataTypeEnum::IntType(context.i8_type()),
            DataType::I16 => BasicMetadataTypeEnum::IntType(context.i16_type()),
            DataType::I32 => BasicMetadataTypeEnum::IntType(context.i32_type()),
            DataType::I64 => BasicMetadataTypeEnum::IntType(context.i64_type()),
            DataType::U8 => todo!("u8"),
            DataType::U16 => todo!("u16"),
            DataType::U32 => todo!("u32"),
            DataType::U64 => todo!("u64"),
            DataType::F32 => BasicMetadataTypeEnum::FloatType(context.f32_type()),
            DataType::F64 => BasicMetadataTypeEnum::FloatType(context.f64_type()),
            DataType::Bool => BasicMetadataTypeEnum::IntType(context.bool_type()),
            DataType::String => BasicMetadataTypeEnum::PointerType(context.i8_type().ptr_type(inkwell::AddressSpace::Global)),
            DataType::Custom(dt, v) => {
                let mut types: Vec<BasicTypeEnum> = Vec::new();
                for dt in dt {
                    types.push(dt.into_basic_type(context));
                }
                let type_name = context.struct_type(&types, false);
                BasicMetadataTypeEnum::StructType(type_name)
            }
            _ => panic!("Type {:?} cannot be converted into BasicType", self), // TODO: change this message to be more meaningful
        }
    }

    pub fn into_fn_type<'a>(&self, context: &'a Context, args: Vec<DataType>, isVarArgs: bool) -> FunctionType<'a> {
        let t = self.into_llvm_type(context);
        let mut types: Vec<BasicMetadataTypeEnum<'a>> = Vec::new();
        for arg in args {
            types.push(arg.into_basic_metadata_type(context));
        }

        
        match t {
            AnyTypeEnum::VoidType(t) => t.fn_type(types.as_slice(), isVarArgs),
            AnyTypeEnum::StructType(t) => t.fn_type(types.as_slice(), isVarArgs),
            AnyTypeEnum::IntType(t) => t.fn_type(types.as_slice(), isVarArgs),
            AnyTypeEnum::FloatType(t) => t.fn_type(types.as_slice(), isVarArgs),
            AnyTypeEnum::PointerType(t) => t.fn_type(types.as_slice(), isVarArgs),
            AnyTypeEnum::VoidType(t) => t.fn_type(types.as_slice(), isVarArgs),
            _ => panic!("Unknown type") // TODO: Make this more meaningful
        }
    }
}

pub trait ToBasic {
    fn to_basic(&self) -> BasicValueEnum;
}

impl<'ctx> ToBasic for AnyValueEnum<'ctx> {
    fn to_basic(&self) -> BasicValueEnum {
        match self {
            AnyValueEnum::IntValue(v) => BasicValueEnum::IntValue(*v),
            AnyValueEnum::FloatValue(v) => BasicValueEnum::FloatValue(*v),
            AnyValueEnum::PointerValue(v) => BasicValueEnum::PointerValue(*v),
            AnyValueEnum::StructValue(v) => BasicValueEnum::StructValue(*v),
            AnyValueEnum::VectorValue(v) => BasicValueEnum::VectorValue(*v),
            _ => panic!("Cannot convert {:?} to BasicValue", self) // TODO: Make this more meaningful
        }
    }
}

pub trait ToAny {
    fn to_any(&self) -> AnyValueEnum;
}

impl<'ctx> ToAny for BasicValueEnum<'ctx> {
    fn to_any(&self) -> AnyValueEnum {
        match self {
            BasicValueEnum::IntValue(v) => AnyValueEnum::IntValue(v.clone()),
            BasicValueEnum::FloatValue(v) => AnyValueEnum::FloatValue(v.clone()),
            BasicValueEnum::PointerValue(v) => AnyValueEnum::PointerValue(v.clone()),
            BasicValueEnum::StructValue(v) => AnyValueEnum::StructValue(v.clone()),
            BasicValueEnum::VectorValue(v) => AnyValueEnum::VectorValue(v.clone()),
            _ => panic!("Cannot convert {:?} to AnyValue", self) // TODO: Make this more meaningful
        }
    }
}