use crate::descriptor::FileDescriptorProto;
use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;

pub(crate) enum MessageOrEnum<'a> {
    Message(&'a DescriptorProto),
    Enum(&'a EnumDescriptorProto),
}

impl<'a> MessageOrEnum<'a> {
    fn from_two_options(m: Option<&'a DescriptorProto>, e: Option<&'a EnumDescriptorProto>)
        -> MessageOrEnum<'a>
    {
        match (m, e) {
            (Some(_), Some(_)) => panic!("enum and message with the same name"),
            (Some(m), None) => MessageOrEnum::Message(m),
            (None, Some(e)) => MessageOrEnum::Enum(e),
            (None, None) => panic!("not found"),
        }
    }
}

pub(crate) fn find_message_or_enum<'a>(file: &'a FileDescriptorProto, full_name: &str)
    -> (String, MessageOrEnum<'a>)
{
    let mut path = full_name.split('.');
    let first = path.next().unwrap();
    let child_message = file.message_type.iter().find(|m| m.get_name() == first);
    let child_enum = file.enum_type.iter().find(|e| e.get_name() == first);

    let mut package_to_name = String::new();
    let mut me = MessageOrEnum::from_two_options(child_message, child_enum);

    for name in path {
        let message = match me {
            MessageOrEnum::Message(m) => m,
            MessageOrEnum::Enum(_) => panic!("enum has no children"),
        };

        if !package_to_name.is_empty() {
            package_to_name.push_str(".");
        }
        package_to_name.push_str(message.get_name());

        let child_message = message.nested_type.iter().find(|m| m.get_name() == name);
        let child_enum = message.enum_type.iter().find(|e| e.get_name() == name);
        me = MessageOrEnum::from_two_options(child_message, child_enum)
    }

    (package_to_name, me)
}
