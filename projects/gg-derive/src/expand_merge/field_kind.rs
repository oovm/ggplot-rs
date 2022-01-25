use super::*;

pub enum MergeFieldKind {
    Option(Type),
}

impl MergeFieldKind {
    pub fn parse(ty: &Type) -> Option<Self> {
        match ty {
            Type::Path(type_path) if type_path.qself.is_none() && path_is_option(&type_path.path) => {
                Some(Self::Option(extract_type(type_path)?))
            }
            _ => None,
        }
    }
    pub fn as_getter_type(&self) -> &Type {
        match self {
            Self::Option(t) => t,
        }
    }
    pub fn as_setter_type(&self) -> &Type {
        match self {
            Self::Option(t) => t,
        }
    }
    pub fn as_builder_type(&self) -> &Type {
        match self {
            Self::Option(t) => t,
        }
    }
}

fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none() && path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Option"
}

fn extract_type(type_path: &TypePath) -> Option<Type> {
    // Get the first segment of the path (there is only one, in fact: "Option"):
    let type_params = &type_path.path.segments.iter().next()?.arguments;
    // It should have only on angle-bracketed param ("<String>"):
    let generic_arg = match type_params {
        PathArguments::AngleBracketed(params) => params.args.iter().next()?,
        _ => return None,
    };
    // This argument must be a type:
    match generic_arg {
        GenericArgument::Type(ty) => Some(ty.to_owned()),
        _ => return None,
    }
}
