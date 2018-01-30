// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Event {
    // message fields
    field_type: ::std::option::Option<Event_Type>,
    subscribed: ::protobuf::SingularPtrField<Event_Subscribed>,
    offers: ::protobuf::SingularPtrField<Event_Offers>,
    inverse_offers: ::protobuf::SingularPtrField<Event_InverseOffers>,
    rescind: ::protobuf::SingularPtrField<Event_Rescind>,
    rescind_inverse_offer: ::protobuf::SingularPtrField<Event_RescindInverseOffer>,
    update: ::protobuf::SingularPtrField<Event_Update>,
    update_operation_status: ::protobuf::SingularPtrField<Event_UpdateOperationStatus>,
    message: ::protobuf::SingularPtrField<Event_Message>,
    failure: ::protobuf::SingularPtrField<Event_Failure>,
    error: ::protobuf::SingularPtrField<Event_Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event {}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(Event::new)
        }
    }

    // optional .mesos.v1.scheduler.Event.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Event_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Event_Type {
        self.field_type.unwrap_or(Event_Type::UNKNOWN)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<Event_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<Event_Type> {
        &mut self.field_type
    }

    // optional .mesos.v1.scheduler.Event.Subscribed subscribed = 2;

    pub fn clear_subscribed(&mut self) {
        self.subscribed.clear();
    }

    pub fn has_subscribed(&self) -> bool {
        self.subscribed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscribed(&mut self, v: Event_Subscribed) {
        self.subscribed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscribed(&mut self) -> &mut Event_Subscribed {
        if self.subscribed.is_none() {
            self.subscribed.set_default();
        }
        self.subscribed.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscribed(&mut self) -> Event_Subscribed {
        self.subscribed.take().unwrap_or_else(|| Event_Subscribed::new())
    }

    pub fn get_subscribed(&self) -> &Event_Subscribed {
        self.subscribed.as_ref().unwrap_or_else(|| Event_Subscribed::default_instance())
    }

    fn get_subscribed_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Subscribed> {
        &self.subscribed
    }

    fn mut_subscribed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Subscribed> {
        &mut self.subscribed
    }

    // optional .mesos.v1.scheduler.Event.Offers offers = 3;

    pub fn clear_offers(&mut self) {
        self.offers.clear();
    }

    pub fn has_offers(&self) -> bool {
        self.offers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offers(&mut self, v: Event_Offers) {
        self.offers = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_offers(&mut self) -> &mut Event_Offers {
        if self.offers.is_none() {
            self.offers.set_default();
        }
        self.offers.as_mut().unwrap()
    }

    // Take field
    pub fn take_offers(&mut self) -> Event_Offers {
        self.offers.take().unwrap_or_else(|| Event_Offers::new())
    }

    pub fn get_offers(&self) -> &Event_Offers {
        self.offers.as_ref().unwrap_or_else(|| Event_Offers::default_instance())
    }

    fn get_offers_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Offers> {
        &self.offers
    }

    fn mut_offers_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Offers> {
        &mut self.offers
    }

    // optional .mesos.v1.scheduler.Event.InverseOffers inverse_offers = 9;

    pub fn clear_inverse_offers(&mut self) {
        self.inverse_offers.clear();
    }

    pub fn has_inverse_offers(&self) -> bool {
        self.inverse_offers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inverse_offers(&mut self, v: Event_InverseOffers) {
        self.inverse_offers = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inverse_offers(&mut self) -> &mut Event_InverseOffers {
        if self.inverse_offers.is_none() {
            self.inverse_offers.set_default();
        }
        self.inverse_offers.as_mut().unwrap()
    }

    // Take field
    pub fn take_inverse_offers(&mut self) -> Event_InverseOffers {
        self.inverse_offers.take().unwrap_or_else(|| Event_InverseOffers::new())
    }

    pub fn get_inverse_offers(&self) -> &Event_InverseOffers {
        self.inverse_offers.as_ref().unwrap_or_else(|| Event_InverseOffers::default_instance())
    }

    fn get_inverse_offers_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_InverseOffers> {
        &self.inverse_offers
    }

    fn mut_inverse_offers_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_InverseOffers> {
        &mut self.inverse_offers
    }

    // optional .mesos.v1.scheduler.Event.Rescind rescind = 4;

    pub fn clear_rescind(&mut self) {
        self.rescind.clear();
    }

    pub fn has_rescind(&self) -> bool {
        self.rescind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rescind(&mut self, v: Event_Rescind) {
        self.rescind = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rescind(&mut self) -> &mut Event_Rescind {
        if self.rescind.is_none() {
            self.rescind.set_default();
        }
        self.rescind.as_mut().unwrap()
    }

    // Take field
    pub fn take_rescind(&mut self) -> Event_Rescind {
        self.rescind.take().unwrap_or_else(|| Event_Rescind::new())
    }

    pub fn get_rescind(&self) -> &Event_Rescind {
        self.rescind.as_ref().unwrap_or_else(|| Event_Rescind::default_instance())
    }

    fn get_rescind_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Rescind> {
        &self.rescind
    }

    fn mut_rescind_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Rescind> {
        &mut self.rescind
    }

    // optional .mesos.v1.scheduler.Event.RescindInverseOffer rescind_inverse_offer = 10;

    pub fn clear_rescind_inverse_offer(&mut self) {
        self.rescind_inverse_offer.clear();
    }

    pub fn has_rescind_inverse_offer(&self) -> bool {
        self.rescind_inverse_offer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rescind_inverse_offer(&mut self, v: Event_RescindInverseOffer) {
        self.rescind_inverse_offer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rescind_inverse_offer(&mut self) -> &mut Event_RescindInverseOffer {
        if self.rescind_inverse_offer.is_none() {
            self.rescind_inverse_offer.set_default();
        }
        self.rescind_inverse_offer.as_mut().unwrap()
    }

    // Take field
    pub fn take_rescind_inverse_offer(&mut self) -> Event_RescindInverseOffer {
        self.rescind_inverse_offer.take().unwrap_or_else(|| Event_RescindInverseOffer::new())
    }

    pub fn get_rescind_inverse_offer(&self) -> &Event_RescindInverseOffer {
        self.rescind_inverse_offer.as_ref().unwrap_or_else(|| Event_RescindInverseOffer::default_instance())
    }

    fn get_rescind_inverse_offer_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_RescindInverseOffer> {
        &self.rescind_inverse_offer
    }

    fn mut_rescind_inverse_offer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_RescindInverseOffer> {
        &mut self.rescind_inverse_offer
    }

    // optional .mesos.v1.scheduler.Event.Update update = 5;

    pub fn clear_update(&mut self) {
        self.update.clear();
    }

    pub fn has_update(&self) -> bool {
        self.update.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update(&mut self, v: Event_Update) {
        self.update = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update(&mut self) -> &mut Event_Update {
        if self.update.is_none() {
            self.update.set_default();
        }
        self.update.as_mut().unwrap()
    }

    // Take field
    pub fn take_update(&mut self) -> Event_Update {
        self.update.take().unwrap_or_else(|| Event_Update::new())
    }

    pub fn get_update(&self) -> &Event_Update {
        self.update.as_ref().unwrap_or_else(|| Event_Update::default_instance())
    }

    fn get_update_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Update> {
        &self.update
    }

    fn mut_update_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Update> {
        &mut self.update
    }

    // optional .mesos.v1.scheduler.Event.UpdateOperationStatus update_operation_status = 11;

    pub fn clear_update_operation_status(&mut self) {
        self.update_operation_status.clear();
    }

    pub fn has_update_operation_status(&self) -> bool {
        self.update_operation_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_operation_status(&mut self, v: Event_UpdateOperationStatus) {
        self.update_operation_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_operation_status(&mut self) -> &mut Event_UpdateOperationStatus {
        if self.update_operation_status.is_none() {
            self.update_operation_status.set_default();
        }
        self.update_operation_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_operation_status(&mut self) -> Event_UpdateOperationStatus {
        self.update_operation_status.take().unwrap_or_else(|| Event_UpdateOperationStatus::new())
    }

    pub fn get_update_operation_status(&self) -> &Event_UpdateOperationStatus {
        self.update_operation_status.as_ref().unwrap_or_else(|| Event_UpdateOperationStatus::default_instance())
    }

    fn get_update_operation_status_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_UpdateOperationStatus> {
        &self.update_operation_status
    }

    fn mut_update_operation_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_UpdateOperationStatus> {
        &mut self.update_operation_status
    }

    // optional .mesos.v1.scheduler.Event.Message message = 6;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: Event_Message) {
        self.message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut Event_Message {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> Event_Message {
        self.message.take().unwrap_or_else(|| Event_Message::new())
    }

    pub fn get_message(&self) -> &Event_Message {
        self.message.as_ref().unwrap_or_else(|| Event_Message::default_instance())
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Message> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Message> {
        &mut self.message
    }

    // optional .mesos.v1.scheduler.Event.Failure failure = 7;

    pub fn clear_failure(&mut self) {
        self.failure.clear();
    }

    pub fn has_failure(&self) -> bool {
        self.failure.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failure(&mut self, v: Event_Failure) {
        self.failure = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_failure(&mut self) -> &mut Event_Failure {
        if self.failure.is_none() {
            self.failure.set_default();
        }
        self.failure.as_mut().unwrap()
    }

    // Take field
    pub fn take_failure(&mut self) -> Event_Failure {
        self.failure.take().unwrap_or_else(|| Event_Failure::new())
    }

    pub fn get_failure(&self) -> &Event_Failure {
        self.failure.as_ref().unwrap_or_else(|| Event_Failure::default_instance())
    }

    fn get_failure_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Failure> {
        &self.failure
    }

    fn mut_failure_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Failure> {
        &mut self.failure
    }

    // optional .mesos.v1.scheduler.Event.Error error = 8;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Event_Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut Event_Error {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Event_Error {
        self.error.take().unwrap_or_else(|| Event_Error::new())
    }

    pub fn get_error(&self) -> &Event_Error {
        self.error.as_ref().unwrap_or_else(|| Event_Error::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<Event_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event_Error> {
        &mut self.error
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        for v in &self.subscribed {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.offers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.inverse_offers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rescind {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rescind_inverse_offer {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.update {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.update_operation_status {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.message {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.failure {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.subscribed)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.offers)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inverse_offers)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rescind)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rescind_inverse_offer)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_operation_status)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.failure)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.subscribed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.offers.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.inverse_offers.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rescind.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rescind_inverse_offer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.update.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.update_operation_status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.failure.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.subscribed.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.offers.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.inverse_offers.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rescind.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rescind_inverse_offer.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.update.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.update_operation_status.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.failure.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event {
    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Event_Type>>(
                    "type",
                    Event::get_field_type_for_reflect,
                    Event::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Subscribed>>(
                    "subscribed",
                    Event::get_subscribed_for_reflect,
                    Event::mut_subscribed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Offers>>(
                    "offers",
                    Event::get_offers_for_reflect,
                    Event::mut_offers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_InverseOffers>>(
                    "inverse_offers",
                    Event::get_inverse_offers_for_reflect,
                    Event::mut_inverse_offers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Rescind>>(
                    "rescind",
                    Event::get_rescind_for_reflect,
                    Event::mut_rescind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_RescindInverseOffer>>(
                    "rescind_inverse_offer",
                    Event::get_rescind_inverse_offer_for_reflect,
                    Event::mut_rescind_inverse_offer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Update>>(
                    "update",
                    Event::get_update_for_reflect,
                    Event::mut_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_UpdateOperationStatus>>(
                    "update_operation_status",
                    Event::get_update_operation_status_for_reflect,
                    Event::mut_update_operation_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Message>>(
                    "message",
                    Event::get_message_for_reflect,
                    Event::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Failure>>(
                    "failure",
                    Event::get_failure_for_reflect,
                    Event::mut_failure_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Error>>(
                    "error",
                    Event::get_error_for_reflect,
                    Event::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_subscribed();
        self.clear_offers();
        self.clear_inverse_offers();
        self.clear_rescind();
        self.clear_rescind_inverse_offer();
        self.clear_update();
        self.clear_update_operation_status();
        self.clear_message();
        self.clear_failure();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Subscribed {
    // message fields
    framework_id: ::protobuf::SingularPtrField<super::mesos::FrameworkID>,
    heartbeat_interval_seconds: ::std::option::Option<f64>,
    master_info: ::protobuf::SingularPtrField<super::mesos::MasterInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Subscribed {}

impl Event_Subscribed {
    pub fn new() -> Event_Subscribed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Subscribed {
        static mut instance: ::protobuf::lazy::Lazy<Event_Subscribed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Subscribed,
        };
        unsafe {
            instance.get(Event_Subscribed::new)
        }
    }

    // required .mesos.v1.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: super::mesos::FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id(&mut self) -> &mut super::mesos::FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        }
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> super::mesos::FrameworkID {
        self.framework_id.take().unwrap_or_else(|| super::mesos::FrameworkID::new())
    }

    pub fn get_framework_id(&self) -> &super::mesos::FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| super::mesos::FrameworkID::default_instance())
    }

    fn get_framework_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::FrameworkID> {
        &self.framework_id
    }

    fn mut_framework_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::FrameworkID> {
        &mut self.framework_id
    }

    // optional double heartbeat_interval_seconds = 2;

    pub fn clear_heartbeat_interval_seconds(&mut self) {
        self.heartbeat_interval_seconds = ::std::option::Option::None;
    }

    pub fn has_heartbeat_interval_seconds(&self) -> bool {
        self.heartbeat_interval_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_heartbeat_interval_seconds(&mut self, v: f64) {
        self.heartbeat_interval_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_heartbeat_interval_seconds(&self) -> f64 {
        self.heartbeat_interval_seconds.unwrap_or(0.)
    }

    fn get_heartbeat_interval_seconds_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.heartbeat_interval_seconds
    }

    fn mut_heartbeat_interval_seconds_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.heartbeat_interval_seconds
    }

    // optional .mesos.v1.MasterInfo master_info = 3;

    pub fn clear_master_info(&mut self) {
        self.master_info.clear();
    }

    pub fn has_master_info(&self) -> bool {
        self.master_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_master_info(&mut self, v: super::mesos::MasterInfo) {
        self.master_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_master_info(&mut self) -> &mut super::mesos::MasterInfo {
        if self.master_info.is_none() {
            self.master_info.set_default();
        }
        self.master_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_master_info(&mut self) -> super::mesos::MasterInfo {
        self.master_info.take().unwrap_or_else(|| super::mesos::MasterInfo::new())
    }

    pub fn get_master_info(&self) -> &super::mesos::MasterInfo {
        self.master_info.as_ref().unwrap_or_else(|| super::mesos::MasterInfo::default_instance())
    }

    fn get_master_info_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::MasterInfo> {
        &self.master_info
    }

    fn mut_master_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::MasterInfo> {
        &mut self.master_info
    }
}

impl ::protobuf::Message for Event_Subscribed {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        }
        for v in &self.framework_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.master_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.framework_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.heartbeat_interval_seconds = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.master_info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.framework_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.heartbeat_interval_seconds {
            my_size += 9;
        }
        if let Some(ref v) = self.master_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.framework_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.heartbeat_interval_seconds {
            os.write_double(2, v)?;
        }
        if let Some(ref v) = self.master_info.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Subscribed {
    fn new() -> Event_Subscribed {
        Event_Subscribed::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Subscribed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::FrameworkID>>(
                    "framework_id",
                    Event_Subscribed::get_framework_id_for_reflect,
                    Event_Subscribed::mut_framework_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "heartbeat_interval_seconds",
                    Event_Subscribed::get_heartbeat_interval_seconds_for_reflect,
                    Event_Subscribed::mut_heartbeat_interval_seconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::MasterInfo>>(
                    "master_info",
                    Event_Subscribed::get_master_info_for_reflect,
                    Event_Subscribed::mut_master_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Subscribed>(
                    "Event_Subscribed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Subscribed {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_heartbeat_interval_seconds();
        self.clear_master_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Subscribed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Subscribed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Offers {
    // message fields
    offers: ::protobuf::RepeatedField<super::mesos::Offer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Offers {}

impl Event_Offers {
    pub fn new() -> Event_Offers {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Offers {
        static mut instance: ::protobuf::lazy::Lazy<Event_Offers> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Offers,
        };
        unsafe {
            instance.get(Event_Offers::new)
        }
    }

    // repeated .mesos.v1.Offer offers = 1;

    pub fn clear_offers(&mut self) {
        self.offers.clear();
    }

    // Param is passed by value, moved
    pub fn set_offers(&mut self, v: ::protobuf::RepeatedField<super::mesos::Offer>) {
        self.offers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_offers(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::Offer> {
        &mut self.offers
    }

    // Take field
    pub fn take_offers(&mut self) -> ::protobuf::RepeatedField<super::mesos::Offer> {
        ::std::mem::replace(&mut self.offers, ::protobuf::RepeatedField::new())
    }

    pub fn get_offers(&self) -> &[super::mesos::Offer] {
        &self.offers
    }

    fn get_offers_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::Offer> {
        &self.offers
    }

    fn mut_offers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::Offer> {
        &mut self.offers
    }
}

impl ::protobuf::Message for Event_Offers {
    fn is_initialized(&self) -> bool {
        for v in &self.offers {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.offers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.offers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.offers {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Offers {
    fn new() -> Event_Offers {
        Event_Offers::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Offers>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Offer>>(
                    "offers",
                    Event_Offers::get_offers_for_reflect,
                    Event_Offers::mut_offers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Offers>(
                    "Event_Offers",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Offers {
    fn clear(&mut self) {
        self.clear_offers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Offers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Offers {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_InverseOffers {
    // message fields
    inverse_offers: ::protobuf::RepeatedField<super::mesos::InverseOffer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_InverseOffers {}

impl Event_InverseOffers {
    pub fn new() -> Event_InverseOffers {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_InverseOffers {
        static mut instance: ::protobuf::lazy::Lazy<Event_InverseOffers> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_InverseOffers,
        };
        unsafe {
            instance.get(Event_InverseOffers::new)
        }
    }

    // repeated .mesos.v1.InverseOffer inverse_offers = 1;

    pub fn clear_inverse_offers(&mut self) {
        self.inverse_offers.clear();
    }

    // Param is passed by value, moved
    pub fn set_inverse_offers(&mut self, v: ::protobuf::RepeatedField<super::mesos::InverseOffer>) {
        self.inverse_offers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inverse_offers(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::InverseOffer> {
        &mut self.inverse_offers
    }

    // Take field
    pub fn take_inverse_offers(&mut self) -> ::protobuf::RepeatedField<super::mesos::InverseOffer> {
        ::std::mem::replace(&mut self.inverse_offers, ::protobuf::RepeatedField::new())
    }

    pub fn get_inverse_offers(&self) -> &[super::mesos::InverseOffer] {
        &self.inverse_offers
    }

    fn get_inverse_offers_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::InverseOffer> {
        &self.inverse_offers
    }

    fn mut_inverse_offers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::InverseOffer> {
        &mut self.inverse_offers
    }
}

impl ::protobuf::Message for Event_InverseOffers {
    fn is_initialized(&self) -> bool {
        for v in &self.inverse_offers {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inverse_offers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.inverse_offers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.inverse_offers {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_InverseOffers {
    fn new() -> Event_InverseOffers {
        Event_InverseOffers::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_InverseOffers>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::InverseOffer>>(
                    "inverse_offers",
                    Event_InverseOffers::get_inverse_offers_for_reflect,
                    Event_InverseOffers::mut_inverse_offers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_InverseOffers>(
                    "Event_InverseOffers",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_InverseOffers {
    fn clear(&mut self) {
        self.clear_inverse_offers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_InverseOffers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_InverseOffers {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Rescind {
    // message fields
    offer_id: ::protobuf::SingularPtrField<super::mesos::OfferID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Rescind {}

impl Event_Rescind {
    pub fn new() -> Event_Rescind {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Rescind {
        static mut instance: ::protobuf::lazy::Lazy<Event_Rescind> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Rescind,
        };
        unsafe {
            instance.get(Event_Rescind::new)
        }
    }

    // required .mesos.v1.OfferID offer_id = 1;

    pub fn clear_offer_id(&mut self) {
        self.offer_id.clear();
    }

    pub fn has_offer_id(&self) -> bool {
        self.offer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offer_id(&mut self, v: super::mesos::OfferID) {
        self.offer_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_offer_id(&mut self) -> &mut super::mesos::OfferID {
        if self.offer_id.is_none() {
            self.offer_id.set_default();
        }
        self.offer_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_offer_id(&mut self) -> super::mesos::OfferID {
        self.offer_id.take().unwrap_or_else(|| super::mesos::OfferID::new())
    }

    pub fn get_offer_id(&self) -> &super::mesos::OfferID {
        self.offer_id.as_ref().unwrap_or_else(|| super::mesos::OfferID::default_instance())
    }

    fn get_offer_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::OfferID> {
        &self.offer_id
    }

    fn mut_offer_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::OfferID> {
        &mut self.offer_id
    }
}

impl ::protobuf::Message for Event_Rescind {
    fn is_initialized(&self) -> bool {
        if self.offer_id.is_none() {
            return false;
        }
        for v in &self.offer_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.offer_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.offer_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.offer_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Rescind {
    fn new() -> Event_Rescind {
        Event_Rescind::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Rescind>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OfferID>>(
                    "offer_id",
                    Event_Rescind::get_offer_id_for_reflect,
                    Event_Rescind::mut_offer_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Rescind>(
                    "Event_Rescind",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Rescind {
    fn clear(&mut self) {
        self.clear_offer_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Rescind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Rescind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_RescindInverseOffer {
    // message fields
    inverse_offer_id: ::protobuf::SingularPtrField<super::mesos::OfferID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_RescindInverseOffer {}

impl Event_RescindInverseOffer {
    pub fn new() -> Event_RescindInverseOffer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_RescindInverseOffer {
        static mut instance: ::protobuf::lazy::Lazy<Event_RescindInverseOffer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_RescindInverseOffer,
        };
        unsafe {
            instance.get(Event_RescindInverseOffer::new)
        }
    }

    // required .mesos.v1.OfferID inverse_offer_id = 1;

    pub fn clear_inverse_offer_id(&mut self) {
        self.inverse_offer_id.clear();
    }

    pub fn has_inverse_offer_id(&self) -> bool {
        self.inverse_offer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inverse_offer_id(&mut self, v: super::mesos::OfferID) {
        self.inverse_offer_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inverse_offer_id(&mut self) -> &mut super::mesos::OfferID {
        if self.inverse_offer_id.is_none() {
            self.inverse_offer_id.set_default();
        }
        self.inverse_offer_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_inverse_offer_id(&mut self) -> super::mesos::OfferID {
        self.inverse_offer_id.take().unwrap_or_else(|| super::mesos::OfferID::new())
    }

    pub fn get_inverse_offer_id(&self) -> &super::mesos::OfferID {
        self.inverse_offer_id.as_ref().unwrap_or_else(|| super::mesos::OfferID::default_instance())
    }

    fn get_inverse_offer_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::OfferID> {
        &self.inverse_offer_id
    }

    fn mut_inverse_offer_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::OfferID> {
        &mut self.inverse_offer_id
    }
}

impl ::protobuf::Message for Event_RescindInverseOffer {
    fn is_initialized(&self) -> bool {
        if self.inverse_offer_id.is_none() {
            return false;
        }
        for v in &self.inverse_offer_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inverse_offer_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.inverse_offer_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.inverse_offer_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_RescindInverseOffer {
    fn new() -> Event_RescindInverseOffer {
        Event_RescindInverseOffer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_RescindInverseOffer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OfferID>>(
                    "inverse_offer_id",
                    Event_RescindInverseOffer::get_inverse_offer_id_for_reflect,
                    Event_RescindInverseOffer::mut_inverse_offer_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_RescindInverseOffer>(
                    "Event_RescindInverseOffer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_RescindInverseOffer {
    fn clear(&mut self) {
        self.clear_inverse_offer_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_RescindInverseOffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_RescindInverseOffer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Update {
    // message fields
    status: ::protobuf::SingularPtrField<super::mesos::TaskStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Update {}

impl Event_Update {
    pub fn new() -> Event_Update {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Update {
        static mut instance: ::protobuf::lazy::Lazy<Event_Update> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Update,
        };
        unsafe {
            instance.get(Event_Update::new)
        }
    }

    // required .mesos.v1.TaskStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::mesos::TaskStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::mesos::TaskStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::mesos::TaskStatus {
        self.status.take().unwrap_or_else(|| super::mesos::TaskStatus::new())
    }

    pub fn get_status(&self) -> &super::mesos::TaskStatus {
        self.status.as_ref().unwrap_or_else(|| super::mesos::TaskStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::TaskStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::TaskStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for Event_Update {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Update {
    fn new() -> Event_Update {
        Event_Update::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Update>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::TaskStatus>>(
                    "status",
                    Event_Update::get_status_for_reflect,
                    Event_Update::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Update>(
                    "Event_Update",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Update {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Update {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Update {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_UpdateOperationStatus {
    // message fields
    status: ::protobuf::SingularPtrField<super::mesos::OperationStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_UpdateOperationStatus {}

impl Event_UpdateOperationStatus {
    pub fn new() -> Event_UpdateOperationStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_UpdateOperationStatus {
        static mut instance: ::protobuf::lazy::Lazy<Event_UpdateOperationStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_UpdateOperationStatus,
        };
        unsafe {
            instance.get(Event_UpdateOperationStatus::new)
        }
    }

    // required .mesos.v1.OperationStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::mesos::OperationStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::mesos::OperationStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::mesos::OperationStatus {
        self.status.take().unwrap_or_else(|| super::mesos::OperationStatus::new())
    }

    pub fn get_status(&self) -> &super::mesos::OperationStatus {
        self.status.as_ref().unwrap_or_else(|| super::mesos::OperationStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::OperationStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::OperationStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for Event_UpdateOperationStatus {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_UpdateOperationStatus {
    fn new() -> Event_UpdateOperationStatus {
        Event_UpdateOperationStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_UpdateOperationStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OperationStatus>>(
                    "status",
                    Event_UpdateOperationStatus::get_status_for_reflect,
                    Event_UpdateOperationStatus::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_UpdateOperationStatus>(
                    "Event_UpdateOperationStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_UpdateOperationStatus {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_UpdateOperationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_UpdateOperationStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Message {
    // message fields
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    executor_id: ::protobuf::SingularPtrField<super::mesos::ExecutorID>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Message {}

impl Event_Message {
    pub fn new() -> Event_Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Message {
        static mut instance: ::protobuf::lazy::Lazy<Event_Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Message,
        };
        unsafe {
            instance.get(Event_Message::new)
        }
    }

    // required .mesos.v1.AgentID agent_id = 1;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // required .mesos.v1.ExecutorID executor_id = 2;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: super::mesos::ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id(&mut self) -> &mut super::mesos::ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        }
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> super::mesos::ExecutorID {
        self.executor_id.take().unwrap_or_else(|| super::mesos::ExecutorID::new())
    }

    pub fn get_executor_id(&self) -> &super::mesos::ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| super::mesos::ExecutorID::default_instance())
    }

    fn get_executor_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &self.executor_id
    }

    fn mut_executor_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &mut self.executor_id
    }

    // required bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for Event_Message {
    fn is_initialized(&self) -> bool {
        if self.agent_id.is_none() {
            return false;
        }
        if self.executor_id.is_none() {
            return false;
        }
        if self.data.is_none() {
            return false;
        }
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.executor_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executor_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.executor_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.executor_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Message {
    fn new() -> Event_Message {
        Event_Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Event_Message::get_agent_id_for_reflect,
                    Event_Message::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::ExecutorID>>(
                    "executor_id",
                    Event_Message::get_executor_id_for_reflect,
                    Event_Message::mut_executor_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Event_Message::get_data_for_reflect,
                    Event_Message::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Message>(
                    "Event_Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Message {
    fn clear(&mut self) {
        self.clear_agent_id();
        self.clear_executor_id();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Failure {
    // message fields
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    executor_id: ::protobuf::SingularPtrField<super::mesos::ExecutorID>,
    status: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Failure {}

impl Event_Failure {
    pub fn new() -> Event_Failure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Failure {
        static mut instance: ::protobuf::lazy::Lazy<Event_Failure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Failure,
        };
        unsafe {
            instance.get(Event_Failure::new)
        }
    }

    // optional .mesos.v1.AgentID agent_id = 1;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // optional .mesos.v1.ExecutorID executor_id = 2;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: super::mesos::ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id(&mut self) -> &mut super::mesos::ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        }
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> super::mesos::ExecutorID {
        self.executor_id.take().unwrap_or_else(|| super::mesos::ExecutorID::new())
    }

    pub fn get_executor_id(&self) -> &super::mesos::ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| super::mesos::ExecutorID::default_instance())
    }

    fn get_executor_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &self.executor_id
    }

    fn mut_executor_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &mut self.executor_id
    }

    // optional int32 status = 3;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: i32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> i32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status
    }
}

impl ::protobuf::Message for Event_Failure {
    fn is_initialized(&self) -> bool {
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.executor_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executor_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.executor_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.executor_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.status {
            os.write_int32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Failure {
    fn new() -> Event_Failure {
        Event_Failure::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Failure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Event_Failure::get_agent_id_for_reflect,
                    Event_Failure::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::ExecutorID>>(
                    "executor_id",
                    Event_Failure::get_executor_id_for_reflect,
                    Event_Failure::mut_executor_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "status",
                    Event_Failure::get_status_for_reflect,
                    Event_Failure::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Failure>(
                    "Event_Failure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Failure {
    fn clear(&mut self) {
        self.clear_agent_id();
        self.clear_executor_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Failure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Failure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Error {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event_Error {}

impl Event_Error {
    pub fn new() -> Event_Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event_Error {
        static mut instance: ::protobuf::lazy::Lazy<Event_Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event_Error,
        };
        unsafe {
            instance.get(Event_Error::new)
        }
    }

    // required string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }
}

impl ::protobuf::Message for Event_Error {
    fn is_initialized(&self) -> bool {
        if self.message.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event_Error {
    fn new() -> Event_Error {
        Event_Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event_Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    Event_Error::get_message_for_reflect,
                    Event_Error::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event_Error>(
                    "Event_Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event_Error {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Event_Type {
    UNKNOWN = 0,
    SUBSCRIBED = 1,
    OFFERS = 2,
    INVERSE_OFFERS = 9,
    RESCIND = 3,
    RESCIND_INVERSE_OFFER = 10,
    UPDATE = 4,
    UPDATE_OPERATION_STATUS = 11,
    MESSAGE = 5,
    FAILURE = 6,
    ERROR = 7,
    HEARTBEAT = 8,
}

impl ::protobuf::ProtobufEnum for Event_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Event_Type> {
        match value {
            0 => ::std::option::Option::Some(Event_Type::UNKNOWN),
            1 => ::std::option::Option::Some(Event_Type::SUBSCRIBED),
            2 => ::std::option::Option::Some(Event_Type::OFFERS),
            9 => ::std::option::Option::Some(Event_Type::INVERSE_OFFERS),
            3 => ::std::option::Option::Some(Event_Type::RESCIND),
            10 => ::std::option::Option::Some(Event_Type::RESCIND_INVERSE_OFFER),
            4 => ::std::option::Option::Some(Event_Type::UPDATE),
            11 => ::std::option::Option::Some(Event_Type::UPDATE_OPERATION_STATUS),
            5 => ::std::option::Option::Some(Event_Type::MESSAGE),
            6 => ::std::option::Option::Some(Event_Type::FAILURE),
            7 => ::std::option::Option::Some(Event_Type::ERROR),
            8 => ::std::option::Option::Some(Event_Type::HEARTBEAT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Event_Type] = &[
            Event_Type::UNKNOWN,
            Event_Type::SUBSCRIBED,
            Event_Type::OFFERS,
            Event_Type::INVERSE_OFFERS,
            Event_Type::RESCIND,
            Event_Type::RESCIND_INVERSE_OFFER,
            Event_Type::UPDATE,
            Event_Type::UPDATE_OPERATION_STATUS,
            Event_Type::MESSAGE,
            Event_Type::FAILURE,
            Event_Type::ERROR,
            Event_Type::HEARTBEAT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Event_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Event_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Event_Type {
}

impl ::protobuf::reflect::ProtobufValue for Event_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    reconcile_operations: ::protobuf::SingularPtrField<Response_ReconcileOperations>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // optional .mesos.v1.scheduler.Response.ReconcileOperations reconcile_operations = 1;

    pub fn clear_reconcile_operations(&mut self) {
        self.reconcile_operations.clear();
    }

    pub fn has_reconcile_operations(&self) -> bool {
        self.reconcile_operations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reconcile_operations(&mut self, v: Response_ReconcileOperations) {
        self.reconcile_operations = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reconcile_operations(&mut self) -> &mut Response_ReconcileOperations {
        if self.reconcile_operations.is_none() {
            self.reconcile_operations.set_default();
        }
        self.reconcile_operations.as_mut().unwrap()
    }

    // Take field
    pub fn take_reconcile_operations(&mut self) -> Response_ReconcileOperations {
        self.reconcile_operations.take().unwrap_or_else(|| Response_ReconcileOperations::new())
    }

    pub fn get_reconcile_operations(&self) -> &Response_ReconcileOperations {
        self.reconcile_operations.as_ref().unwrap_or_else(|| Response_ReconcileOperations::default_instance())
    }

    fn get_reconcile_operations_for_reflect(&self) -> &::protobuf::SingularPtrField<Response_ReconcileOperations> {
        &self.reconcile_operations
    }

    fn mut_reconcile_operations_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Response_ReconcileOperations> {
        &mut self.reconcile_operations
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        for v in &self.reconcile_operations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reconcile_operations)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.reconcile_operations.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reconcile_operations.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Response_ReconcileOperations>>(
                    "reconcile_operations",
                    Response::get_reconcile_operations_for_reflect,
                    Response::mut_reconcile_operations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_reconcile_operations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response_ReconcileOperations {
    // message fields
    operation_statuses: ::protobuf::RepeatedField<super::mesos::OperationStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response_ReconcileOperations {}

impl Response_ReconcileOperations {
    pub fn new() -> Response_ReconcileOperations {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response_ReconcileOperations {
        static mut instance: ::protobuf::lazy::Lazy<Response_ReconcileOperations> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response_ReconcileOperations,
        };
        unsafe {
            instance.get(Response_ReconcileOperations::new)
        }
    }

    // repeated .mesos.v1.OperationStatus operation_statuses = 1;

    pub fn clear_operation_statuses(&mut self) {
        self.operation_statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_operation_statuses(&mut self, v: ::protobuf::RepeatedField<super::mesos::OperationStatus>) {
        self.operation_statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_operation_statuses(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OperationStatus> {
        &mut self.operation_statuses
    }

    // Take field
    pub fn take_operation_statuses(&mut self) -> ::protobuf::RepeatedField<super::mesos::OperationStatus> {
        ::std::mem::replace(&mut self.operation_statuses, ::protobuf::RepeatedField::new())
    }

    pub fn get_operation_statuses(&self) -> &[super::mesos::OperationStatus] {
        &self.operation_statuses
    }

    fn get_operation_statuses_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::OperationStatus> {
        &self.operation_statuses
    }

    fn mut_operation_statuses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OperationStatus> {
        &mut self.operation_statuses
    }
}

impl ::protobuf::Message for Response_ReconcileOperations {
    fn is_initialized(&self) -> bool {
        for v in &self.operation_statuses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.operation_statuses)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.operation_statuses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.operation_statuses {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response_ReconcileOperations {
    fn new() -> Response_ReconcileOperations {
        Response_ReconcileOperations::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response_ReconcileOperations>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OperationStatus>>(
                    "operation_statuses",
                    Response_ReconcileOperations::get_operation_statuses_for_reflect,
                    Response_ReconcileOperations::mut_operation_statuses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response_ReconcileOperations>(
                    "Response_ReconcileOperations",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response_ReconcileOperations {
    fn clear(&mut self) {
        self.clear_operation_statuses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response_ReconcileOperations {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response_ReconcileOperations {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Response_Type {
    UNKNOWN = 0,
    RECONCILE_OPERATIONS = 1,
}

impl ::protobuf::ProtobufEnum for Response_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Response_Type> {
        match value {
            0 => ::std::option::Option::Some(Response_Type::UNKNOWN),
            1 => ::std::option::Option::Some(Response_Type::RECONCILE_OPERATIONS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Response_Type] = &[
            Response_Type::UNKNOWN,
            Response_Type::RECONCILE_OPERATIONS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Response_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Response_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Response_Type {
}

impl ::protobuf::reflect::ProtobufValue for Response_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call {
    // message fields
    framework_id: ::protobuf::SingularPtrField<super::mesos::FrameworkID>,
    field_type: ::std::option::Option<Call_Type>,
    subscribe: ::protobuf::SingularPtrField<Call_Subscribe>,
    accept: ::protobuf::SingularPtrField<Call_Accept>,
    decline: ::protobuf::SingularPtrField<Call_Decline>,
    accept_inverse_offers: ::protobuf::SingularPtrField<Call_AcceptInverseOffers>,
    decline_inverse_offers: ::protobuf::SingularPtrField<Call_DeclineInverseOffers>,
    revive: ::protobuf::SingularPtrField<Call_Revive>,
    kill: ::protobuf::SingularPtrField<Call_Kill>,
    shutdown: ::protobuf::SingularPtrField<Call_Shutdown>,
    acknowledge: ::protobuf::SingularPtrField<Call_Acknowledge>,
    acknowledge_operation_status: ::protobuf::SingularPtrField<Call_AcknowledgeOperationStatus>,
    reconcile: ::protobuf::SingularPtrField<Call_Reconcile>,
    reconcile_operations: ::protobuf::SingularPtrField<Call_ReconcileOperations>,
    message: ::protobuf::SingularPtrField<Call_Message>,
    request: ::protobuf::SingularPtrField<Call_Request>,
    suppress: ::protobuf::SingularPtrField<Call_Suppress>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call {}

impl Call {
    pub fn new() -> Call {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call {
        static mut instance: ::protobuf::lazy::Lazy<Call> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call,
        };
        unsafe {
            instance.get(Call::new)
        }
    }

    // optional .mesos.v1.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: super::mesos::FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id(&mut self) -> &mut super::mesos::FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        }
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> super::mesos::FrameworkID {
        self.framework_id.take().unwrap_or_else(|| super::mesos::FrameworkID::new())
    }

    pub fn get_framework_id(&self) -> &super::mesos::FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| super::mesos::FrameworkID::default_instance())
    }

    fn get_framework_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::FrameworkID> {
        &self.framework_id
    }

    fn mut_framework_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::FrameworkID> {
        &mut self.framework_id
    }

    // optional .mesos.v1.scheduler.Call.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Call_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Call_Type {
        self.field_type.unwrap_or(Call_Type::UNKNOWN)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<Call_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<Call_Type> {
        &mut self.field_type
    }

    // optional .mesos.v1.scheduler.Call.Subscribe subscribe = 3;

    pub fn clear_subscribe(&mut self) {
        self.subscribe.clear();
    }

    pub fn has_subscribe(&self) -> bool {
        self.subscribe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscribe(&mut self, v: Call_Subscribe) {
        self.subscribe = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscribe(&mut self) -> &mut Call_Subscribe {
        if self.subscribe.is_none() {
            self.subscribe.set_default();
        }
        self.subscribe.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscribe(&mut self) -> Call_Subscribe {
        self.subscribe.take().unwrap_or_else(|| Call_Subscribe::new())
    }

    pub fn get_subscribe(&self) -> &Call_Subscribe {
        self.subscribe.as_ref().unwrap_or_else(|| Call_Subscribe::default_instance())
    }

    fn get_subscribe_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Subscribe> {
        &self.subscribe
    }

    fn mut_subscribe_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Subscribe> {
        &mut self.subscribe
    }

    // optional .mesos.v1.scheduler.Call.Accept accept = 4;

    pub fn clear_accept(&mut self) {
        self.accept.clear();
    }

    pub fn has_accept(&self) -> bool {
        self.accept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accept(&mut self, v: Call_Accept) {
        self.accept = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accept(&mut self) -> &mut Call_Accept {
        if self.accept.is_none() {
            self.accept.set_default();
        }
        self.accept.as_mut().unwrap()
    }

    // Take field
    pub fn take_accept(&mut self) -> Call_Accept {
        self.accept.take().unwrap_or_else(|| Call_Accept::new())
    }

    pub fn get_accept(&self) -> &Call_Accept {
        self.accept.as_ref().unwrap_or_else(|| Call_Accept::default_instance())
    }

    fn get_accept_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Accept> {
        &self.accept
    }

    fn mut_accept_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Accept> {
        &mut self.accept
    }

    // optional .mesos.v1.scheduler.Call.Decline decline = 5;

    pub fn clear_decline(&mut self) {
        self.decline.clear();
    }

    pub fn has_decline(&self) -> bool {
        self.decline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decline(&mut self, v: Call_Decline) {
        self.decline = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_decline(&mut self) -> &mut Call_Decline {
        if self.decline.is_none() {
            self.decline.set_default();
        }
        self.decline.as_mut().unwrap()
    }

    // Take field
    pub fn take_decline(&mut self) -> Call_Decline {
        self.decline.take().unwrap_or_else(|| Call_Decline::new())
    }

    pub fn get_decline(&self) -> &Call_Decline {
        self.decline.as_ref().unwrap_or_else(|| Call_Decline::default_instance())
    }

    fn get_decline_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Decline> {
        &self.decline
    }

    fn mut_decline_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Decline> {
        &mut self.decline
    }

    // optional .mesos.v1.scheduler.Call.AcceptInverseOffers accept_inverse_offers = 13;

    pub fn clear_accept_inverse_offers(&mut self) {
        self.accept_inverse_offers.clear();
    }

    pub fn has_accept_inverse_offers(&self) -> bool {
        self.accept_inverse_offers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accept_inverse_offers(&mut self, v: Call_AcceptInverseOffers) {
        self.accept_inverse_offers = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accept_inverse_offers(&mut self) -> &mut Call_AcceptInverseOffers {
        if self.accept_inverse_offers.is_none() {
            self.accept_inverse_offers.set_default();
        }
        self.accept_inverse_offers.as_mut().unwrap()
    }

    // Take field
    pub fn take_accept_inverse_offers(&mut self) -> Call_AcceptInverseOffers {
        self.accept_inverse_offers.take().unwrap_or_else(|| Call_AcceptInverseOffers::new())
    }

    pub fn get_accept_inverse_offers(&self) -> &Call_AcceptInverseOffers {
        self.accept_inverse_offers.as_ref().unwrap_or_else(|| Call_AcceptInverseOffers::default_instance())
    }

    fn get_accept_inverse_offers_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_AcceptInverseOffers> {
        &self.accept_inverse_offers
    }

    fn mut_accept_inverse_offers_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_AcceptInverseOffers> {
        &mut self.accept_inverse_offers
    }

    // optional .mesos.v1.scheduler.Call.DeclineInverseOffers decline_inverse_offers = 14;

    pub fn clear_decline_inverse_offers(&mut self) {
        self.decline_inverse_offers.clear();
    }

    pub fn has_decline_inverse_offers(&self) -> bool {
        self.decline_inverse_offers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decline_inverse_offers(&mut self, v: Call_DeclineInverseOffers) {
        self.decline_inverse_offers = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_decline_inverse_offers(&mut self) -> &mut Call_DeclineInverseOffers {
        if self.decline_inverse_offers.is_none() {
            self.decline_inverse_offers.set_default();
        }
        self.decline_inverse_offers.as_mut().unwrap()
    }

    // Take field
    pub fn take_decline_inverse_offers(&mut self) -> Call_DeclineInverseOffers {
        self.decline_inverse_offers.take().unwrap_or_else(|| Call_DeclineInverseOffers::new())
    }

    pub fn get_decline_inverse_offers(&self) -> &Call_DeclineInverseOffers {
        self.decline_inverse_offers.as_ref().unwrap_or_else(|| Call_DeclineInverseOffers::default_instance())
    }

    fn get_decline_inverse_offers_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_DeclineInverseOffers> {
        &self.decline_inverse_offers
    }

    fn mut_decline_inverse_offers_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_DeclineInverseOffers> {
        &mut self.decline_inverse_offers
    }

    // optional .mesos.v1.scheduler.Call.Revive revive = 15;

    pub fn clear_revive(&mut self) {
        self.revive.clear();
    }

    pub fn has_revive(&self) -> bool {
        self.revive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revive(&mut self, v: Call_Revive) {
        self.revive = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revive(&mut self) -> &mut Call_Revive {
        if self.revive.is_none() {
            self.revive.set_default();
        }
        self.revive.as_mut().unwrap()
    }

    // Take field
    pub fn take_revive(&mut self) -> Call_Revive {
        self.revive.take().unwrap_or_else(|| Call_Revive::new())
    }

    pub fn get_revive(&self) -> &Call_Revive {
        self.revive.as_ref().unwrap_or_else(|| Call_Revive::default_instance())
    }

    fn get_revive_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Revive> {
        &self.revive
    }

    fn mut_revive_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Revive> {
        &mut self.revive
    }

    // optional .mesos.v1.scheduler.Call.Kill kill = 6;

    pub fn clear_kill(&mut self) {
        self.kill.clear();
    }

    pub fn has_kill(&self) -> bool {
        self.kill.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kill(&mut self, v: Call_Kill) {
        self.kill = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kill(&mut self) -> &mut Call_Kill {
        if self.kill.is_none() {
            self.kill.set_default();
        }
        self.kill.as_mut().unwrap()
    }

    // Take field
    pub fn take_kill(&mut self) -> Call_Kill {
        self.kill.take().unwrap_or_else(|| Call_Kill::new())
    }

    pub fn get_kill(&self) -> &Call_Kill {
        self.kill.as_ref().unwrap_or_else(|| Call_Kill::default_instance())
    }

    fn get_kill_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Kill> {
        &self.kill
    }

    fn mut_kill_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Kill> {
        &mut self.kill
    }

    // optional .mesos.v1.scheduler.Call.Shutdown shutdown = 7;

    pub fn clear_shutdown(&mut self) {
        self.shutdown.clear();
    }

    pub fn has_shutdown(&self) -> bool {
        self.shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shutdown(&mut self, v: Call_Shutdown) {
        self.shutdown = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shutdown(&mut self) -> &mut Call_Shutdown {
        if self.shutdown.is_none() {
            self.shutdown.set_default();
        }
        self.shutdown.as_mut().unwrap()
    }

    // Take field
    pub fn take_shutdown(&mut self) -> Call_Shutdown {
        self.shutdown.take().unwrap_or_else(|| Call_Shutdown::new())
    }

    pub fn get_shutdown(&self) -> &Call_Shutdown {
        self.shutdown.as_ref().unwrap_or_else(|| Call_Shutdown::default_instance())
    }

    fn get_shutdown_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Shutdown> {
        &self.shutdown
    }

    fn mut_shutdown_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Shutdown> {
        &mut self.shutdown
    }

    // optional .mesos.v1.scheduler.Call.Acknowledge acknowledge = 8;

    pub fn clear_acknowledge(&mut self) {
        self.acknowledge.clear();
    }

    pub fn has_acknowledge(&self) -> bool {
        self.acknowledge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acknowledge(&mut self, v: Call_Acknowledge) {
        self.acknowledge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acknowledge(&mut self) -> &mut Call_Acknowledge {
        if self.acknowledge.is_none() {
            self.acknowledge.set_default();
        }
        self.acknowledge.as_mut().unwrap()
    }

    // Take field
    pub fn take_acknowledge(&mut self) -> Call_Acknowledge {
        self.acknowledge.take().unwrap_or_else(|| Call_Acknowledge::new())
    }

    pub fn get_acknowledge(&self) -> &Call_Acknowledge {
        self.acknowledge.as_ref().unwrap_or_else(|| Call_Acknowledge::default_instance())
    }

    fn get_acknowledge_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Acknowledge> {
        &self.acknowledge
    }

    fn mut_acknowledge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Acknowledge> {
        &mut self.acknowledge
    }

    // optional .mesos.v1.scheduler.Call.AcknowledgeOperationStatus acknowledge_operation_status = 17;

    pub fn clear_acknowledge_operation_status(&mut self) {
        self.acknowledge_operation_status.clear();
    }

    pub fn has_acknowledge_operation_status(&self) -> bool {
        self.acknowledge_operation_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acknowledge_operation_status(&mut self, v: Call_AcknowledgeOperationStatus) {
        self.acknowledge_operation_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acknowledge_operation_status(&mut self) -> &mut Call_AcknowledgeOperationStatus {
        if self.acknowledge_operation_status.is_none() {
            self.acknowledge_operation_status.set_default();
        }
        self.acknowledge_operation_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_acknowledge_operation_status(&mut self) -> Call_AcknowledgeOperationStatus {
        self.acknowledge_operation_status.take().unwrap_or_else(|| Call_AcknowledgeOperationStatus::new())
    }

    pub fn get_acknowledge_operation_status(&self) -> &Call_AcknowledgeOperationStatus {
        self.acknowledge_operation_status.as_ref().unwrap_or_else(|| Call_AcknowledgeOperationStatus::default_instance())
    }

    fn get_acknowledge_operation_status_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_AcknowledgeOperationStatus> {
        &self.acknowledge_operation_status
    }

    fn mut_acknowledge_operation_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_AcknowledgeOperationStatus> {
        &mut self.acknowledge_operation_status
    }

    // optional .mesos.v1.scheduler.Call.Reconcile reconcile = 9;

    pub fn clear_reconcile(&mut self) {
        self.reconcile.clear();
    }

    pub fn has_reconcile(&self) -> bool {
        self.reconcile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reconcile(&mut self, v: Call_Reconcile) {
        self.reconcile = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reconcile(&mut self) -> &mut Call_Reconcile {
        if self.reconcile.is_none() {
            self.reconcile.set_default();
        }
        self.reconcile.as_mut().unwrap()
    }

    // Take field
    pub fn take_reconcile(&mut self) -> Call_Reconcile {
        self.reconcile.take().unwrap_or_else(|| Call_Reconcile::new())
    }

    pub fn get_reconcile(&self) -> &Call_Reconcile {
        self.reconcile.as_ref().unwrap_or_else(|| Call_Reconcile::default_instance())
    }

    fn get_reconcile_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Reconcile> {
        &self.reconcile
    }

    fn mut_reconcile_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Reconcile> {
        &mut self.reconcile
    }

    // optional .mesos.v1.scheduler.Call.ReconcileOperations reconcile_operations = 18;

    pub fn clear_reconcile_operations(&mut self) {
        self.reconcile_operations.clear();
    }

    pub fn has_reconcile_operations(&self) -> bool {
        self.reconcile_operations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reconcile_operations(&mut self, v: Call_ReconcileOperations) {
        self.reconcile_operations = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reconcile_operations(&mut self) -> &mut Call_ReconcileOperations {
        if self.reconcile_operations.is_none() {
            self.reconcile_operations.set_default();
        }
        self.reconcile_operations.as_mut().unwrap()
    }

    // Take field
    pub fn take_reconcile_operations(&mut self) -> Call_ReconcileOperations {
        self.reconcile_operations.take().unwrap_or_else(|| Call_ReconcileOperations::new())
    }

    pub fn get_reconcile_operations(&self) -> &Call_ReconcileOperations {
        self.reconcile_operations.as_ref().unwrap_or_else(|| Call_ReconcileOperations::default_instance())
    }

    fn get_reconcile_operations_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_ReconcileOperations> {
        &self.reconcile_operations
    }

    fn mut_reconcile_operations_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_ReconcileOperations> {
        &mut self.reconcile_operations
    }

    // optional .mesos.v1.scheduler.Call.Message message = 10;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: Call_Message) {
        self.message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut Call_Message {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> Call_Message {
        self.message.take().unwrap_or_else(|| Call_Message::new())
    }

    pub fn get_message(&self) -> &Call_Message {
        self.message.as_ref().unwrap_or_else(|| Call_Message::default_instance())
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Message> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Message> {
        &mut self.message
    }

    // optional .mesos.v1.scheduler.Call.Request request = 11;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: Call_Request) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut Call_Request {
        if self.request.is_none() {
            self.request.set_default();
        }
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> Call_Request {
        self.request.take().unwrap_or_else(|| Call_Request::new())
    }

    pub fn get_request(&self) -> &Call_Request {
        self.request.as_ref().unwrap_or_else(|| Call_Request::default_instance())
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Request> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Request> {
        &mut self.request
    }

    // optional .mesos.v1.scheduler.Call.Suppress suppress = 16;

    pub fn clear_suppress(&mut self) {
        self.suppress.clear();
    }

    pub fn has_suppress(&self) -> bool {
        self.suppress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suppress(&mut self, v: Call_Suppress) {
        self.suppress = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suppress(&mut self) -> &mut Call_Suppress {
        if self.suppress.is_none() {
            self.suppress.set_default();
        }
        self.suppress.as_mut().unwrap()
    }

    // Take field
    pub fn take_suppress(&mut self) -> Call_Suppress {
        self.suppress.take().unwrap_or_else(|| Call_Suppress::new())
    }

    pub fn get_suppress(&self) -> &Call_Suppress {
        self.suppress.as_ref().unwrap_or_else(|| Call_Suppress::default_instance())
    }

    fn get_suppress_for_reflect(&self) -> &::protobuf::SingularPtrField<Call_Suppress> {
        &self.suppress
    }

    fn mut_suppress_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Call_Suppress> {
        &mut self.suppress
    }
}

impl ::protobuf::Message for Call {
    fn is_initialized(&self) -> bool {
        for v in &self.framework_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.subscribe {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.accept {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.decline {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.accept_inverse_offers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.decline_inverse_offers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.revive {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.kill {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.shutdown {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.acknowledge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.acknowledge_operation_status {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reconcile {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reconcile_operations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.message {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.suppress {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.framework_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.subscribe)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.accept)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.decline)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.accept_inverse_offers)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.decline_inverse_offers)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.revive)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kill)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shutdown)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.acknowledge)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.acknowledge_operation_status)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reconcile)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reconcile_operations)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.suppress)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.framework_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.subscribe.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.accept.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.decline.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.accept_inverse_offers.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.decline_inverse_offers.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.revive.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.kill.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.shutdown.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.acknowledge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.acknowledge_operation_status.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.reconcile.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.reconcile_operations.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.suppress.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.framework_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.subscribe.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.accept.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.decline.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.accept_inverse_offers.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.decline_inverse_offers.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.revive.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.kill.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.shutdown.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.acknowledge.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.acknowledge_operation_status.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.reconcile.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.reconcile_operations.as_ref() {
            os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.request.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.suppress.as_ref() {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call {
    fn new() -> Call {
        Call::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::FrameworkID>>(
                    "framework_id",
                    Call::get_framework_id_for_reflect,
                    Call::mut_framework_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Call_Type>>(
                    "type",
                    Call::get_field_type_for_reflect,
                    Call::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Subscribe>>(
                    "subscribe",
                    Call::get_subscribe_for_reflect,
                    Call::mut_subscribe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Accept>>(
                    "accept",
                    Call::get_accept_for_reflect,
                    Call::mut_accept_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Decline>>(
                    "decline",
                    Call::get_decline_for_reflect,
                    Call::mut_decline_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_AcceptInverseOffers>>(
                    "accept_inverse_offers",
                    Call::get_accept_inverse_offers_for_reflect,
                    Call::mut_accept_inverse_offers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_DeclineInverseOffers>>(
                    "decline_inverse_offers",
                    Call::get_decline_inverse_offers_for_reflect,
                    Call::mut_decline_inverse_offers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Revive>>(
                    "revive",
                    Call::get_revive_for_reflect,
                    Call::mut_revive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Kill>>(
                    "kill",
                    Call::get_kill_for_reflect,
                    Call::mut_kill_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Shutdown>>(
                    "shutdown",
                    Call::get_shutdown_for_reflect,
                    Call::mut_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Acknowledge>>(
                    "acknowledge",
                    Call::get_acknowledge_for_reflect,
                    Call::mut_acknowledge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_AcknowledgeOperationStatus>>(
                    "acknowledge_operation_status",
                    Call::get_acknowledge_operation_status_for_reflect,
                    Call::mut_acknowledge_operation_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Reconcile>>(
                    "reconcile",
                    Call::get_reconcile_for_reflect,
                    Call::mut_reconcile_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_ReconcileOperations>>(
                    "reconcile_operations",
                    Call::get_reconcile_operations_for_reflect,
                    Call::mut_reconcile_operations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Message>>(
                    "message",
                    Call::get_message_for_reflect,
                    Call::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Request>>(
                    "request",
                    Call::get_request_for_reflect,
                    Call::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Suppress>>(
                    "suppress",
                    Call::get_suppress_for_reflect,
                    Call::mut_suppress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call>(
                    "Call",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_field_type();
        self.clear_subscribe();
        self.clear_accept();
        self.clear_decline();
        self.clear_accept_inverse_offers();
        self.clear_decline_inverse_offers();
        self.clear_revive();
        self.clear_kill();
        self.clear_shutdown();
        self.clear_acknowledge();
        self.clear_acknowledge_operation_status();
        self.clear_reconcile();
        self.clear_reconcile_operations();
        self.clear_message();
        self.clear_request();
        self.clear_suppress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Subscribe {
    // message fields
    framework_info: ::protobuf::SingularPtrField<super::mesos::FrameworkInfo>,
    suppressed_roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Subscribe {}

impl Call_Subscribe {
    pub fn new() -> Call_Subscribe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Subscribe {
        static mut instance: ::protobuf::lazy::Lazy<Call_Subscribe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Subscribe,
        };
        unsafe {
            instance.get(Call_Subscribe::new)
        }
    }

    // required .mesos.v1.FrameworkInfo framework_info = 1;

    pub fn clear_framework_info(&mut self) {
        self.framework_info.clear();
    }

    pub fn has_framework_info(&self) -> bool {
        self.framework_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_info(&mut self, v: super::mesos::FrameworkInfo) {
        self.framework_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_info(&mut self) -> &mut super::mesos::FrameworkInfo {
        if self.framework_info.is_none() {
            self.framework_info.set_default();
        }
        self.framework_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_info(&mut self) -> super::mesos::FrameworkInfo {
        self.framework_info.take().unwrap_or_else(|| super::mesos::FrameworkInfo::new())
    }

    pub fn get_framework_info(&self) -> &super::mesos::FrameworkInfo {
        self.framework_info.as_ref().unwrap_or_else(|| super::mesos::FrameworkInfo::default_instance())
    }

    fn get_framework_info_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::FrameworkInfo> {
        &self.framework_info
    }

    fn mut_framework_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::FrameworkInfo> {
        &mut self.framework_info
    }

    // repeated string suppressed_roles = 2;

    pub fn clear_suppressed_roles(&mut self) {
        self.suppressed_roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_suppressed_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.suppressed_roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_suppressed_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.suppressed_roles
    }

    // Take field
    pub fn take_suppressed_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.suppressed_roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_suppressed_roles(&self) -> &[::std::string::String] {
        &self.suppressed_roles
    }

    fn get_suppressed_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.suppressed_roles
    }

    fn mut_suppressed_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.suppressed_roles
    }
}

impl ::protobuf::Message for Call_Subscribe {
    fn is_initialized(&self) -> bool {
        if self.framework_info.is_none() {
            return false;
        }
        for v in &self.framework_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.framework_info)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.suppressed_roles)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.framework_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.suppressed_roles {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.framework_info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.suppressed_roles {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Subscribe {
    fn new() -> Call_Subscribe {
        Call_Subscribe::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Subscribe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::FrameworkInfo>>(
                    "framework_info",
                    Call_Subscribe::get_framework_info_for_reflect,
                    Call_Subscribe::mut_framework_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "suppressed_roles",
                    Call_Subscribe::get_suppressed_roles_for_reflect,
                    Call_Subscribe::mut_suppressed_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Subscribe>(
                    "Call_Subscribe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Subscribe {
    fn clear(&mut self) {
        self.clear_framework_info();
        self.clear_suppressed_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Subscribe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Subscribe {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Accept {
    // message fields
    offer_ids: ::protobuf::RepeatedField<super::mesos::OfferID>,
    operations: ::protobuf::RepeatedField<super::mesos::Offer_Operation>,
    filters: ::protobuf::SingularPtrField<super::mesos::Filters>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Accept {}

impl Call_Accept {
    pub fn new() -> Call_Accept {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Accept {
        static mut instance: ::protobuf::lazy::Lazy<Call_Accept> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Accept,
        };
        unsafe {
            instance.get(Call_Accept::new)
        }
    }

    // repeated .mesos.v1.OfferID offer_ids = 1;

    pub fn clear_offer_ids(&mut self) {
        self.offer_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_offer_ids(&mut self, v: ::protobuf::RepeatedField<super::mesos::OfferID>) {
        self.offer_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_offer_ids(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.offer_ids
    }

    // Take field
    pub fn take_offer_ids(&mut self) -> ::protobuf::RepeatedField<super::mesos::OfferID> {
        ::std::mem::replace(&mut self.offer_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_offer_ids(&self) -> &[super::mesos::OfferID] {
        &self.offer_ids
    }

    fn get_offer_ids_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::OfferID> {
        &self.offer_ids
    }

    fn mut_offer_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.offer_ids
    }

    // repeated .mesos.v1.Offer.Operation operations = 2;

    pub fn clear_operations(&mut self) {
        self.operations.clear();
    }

    // Param is passed by value, moved
    pub fn set_operations(&mut self, v: ::protobuf::RepeatedField<super::mesos::Offer_Operation>) {
        self.operations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_operations(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::Offer_Operation> {
        &mut self.operations
    }

    // Take field
    pub fn take_operations(&mut self) -> ::protobuf::RepeatedField<super::mesos::Offer_Operation> {
        ::std::mem::replace(&mut self.operations, ::protobuf::RepeatedField::new())
    }

    pub fn get_operations(&self) -> &[super::mesos::Offer_Operation] {
        &self.operations
    }

    fn get_operations_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::Offer_Operation> {
        &self.operations
    }

    fn mut_operations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::Offer_Operation> {
        &mut self.operations
    }

    // optional .mesos.v1.Filters filters = 3;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    pub fn has_filters(&self) -> bool {
        self.filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: super::mesos::Filters) {
        self.filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filters(&mut self) -> &mut super::mesos::Filters {
        if self.filters.is_none() {
            self.filters.set_default();
        }
        self.filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_filters(&mut self) -> super::mesos::Filters {
        self.filters.take().unwrap_or_else(|| super::mesos::Filters::new())
    }

    pub fn get_filters(&self) -> &super::mesos::Filters {
        self.filters.as_ref().unwrap_or_else(|| super::mesos::Filters::default_instance())
    }

    fn get_filters_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::Filters> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::Filters> {
        &mut self.filters
    }
}

impl ::protobuf::Message for Call_Accept {
    fn is_initialized(&self) -> bool {
        for v in &self.offer_ids {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.operations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.offer_ids)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.operations)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.offer_ids {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.operations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.filters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.offer_ids {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.operations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.filters.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Accept {
    fn new() -> Call_Accept {
        Call_Accept::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Accept>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OfferID>>(
                    "offer_ids",
                    Call_Accept::get_offer_ids_for_reflect,
                    Call_Accept::mut_offer_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Offer_Operation>>(
                    "operations",
                    Call_Accept::get_operations_for_reflect,
                    Call_Accept::mut_operations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Filters>>(
                    "filters",
                    Call_Accept::get_filters_for_reflect,
                    Call_Accept::mut_filters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Accept>(
                    "Call_Accept",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Accept {
    fn clear(&mut self) {
        self.clear_offer_ids();
        self.clear_operations();
        self.clear_filters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Accept {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Accept {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Decline {
    // message fields
    offer_ids: ::protobuf::RepeatedField<super::mesos::OfferID>,
    filters: ::protobuf::SingularPtrField<super::mesos::Filters>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Decline {}

impl Call_Decline {
    pub fn new() -> Call_Decline {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Decline {
        static mut instance: ::protobuf::lazy::Lazy<Call_Decline> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Decline,
        };
        unsafe {
            instance.get(Call_Decline::new)
        }
    }

    // repeated .mesos.v1.OfferID offer_ids = 1;

    pub fn clear_offer_ids(&mut self) {
        self.offer_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_offer_ids(&mut self, v: ::protobuf::RepeatedField<super::mesos::OfferID>) {
        self.offer_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_offer_ids(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.offer_ids
    }

    // Take field
    pub fn take_offer_ids(&mut self) -> ::protobuf::RepeatedField<super::mesos::OfferID> {
        ::std::mem::replace(&mut self.offer_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_offer_ids(&self) -> &[super::mesos::OfferID] {
        &self.offer_ids
    }

    fn get_offer_ids_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::OfferID> {
        &self.offer_ids
    }

    fn mut_offer_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.offer_ids
    }

    // optional .mesos.v1.Filters filters = 2;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    pub fn has_filters(&self) -> bool {
        self.filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: super::mesos::Filters) {
        self.filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filters(&mut self) -> &mut super::mesos::Filters {
        if self.filters.is_none() {
            self.filters.set_default();
        }
        self.filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_filters(&mut self) -> super::mesos::Filters {
        self.filters.take().unwrap_or_else(|| super::mesos::Filters::new())
    }

    pub fn get_filters(&self) -> &super::mesos::Filters {
        self.filters.as_ref().unwrap_or_else(|| super::mesos::Filters::default_instance())
    }

    fn get_filters_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::Filters> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::Filters> {
        &mut self.filters
    }
}

impl ::protobuf::Message for Call_Decline {
    fn is_initialized(&self) -> bool {
        for v in &self.offer_ids {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.offer_ids)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.offer_ids {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.filters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.offer_ids {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.filters.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Decline {
    fn new() -> Call_Decline {
        Call_Decline::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Decline>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OfferID>>(
                    "offer_ids",
                    Call_Decline::get_offer_ids_for_reflect,
                    Call_Decline::mut_offer_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Filters>>(
                    "filters",
                    Call_Decline::get_filters_for_reflect,
                    Call_Decline::mut_filters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Decline>(
                    "Call_Decline",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Decline {
    fn clear(&mut self) {
        self.clear_offer_ids();
        self.clear_filters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Decline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Decline {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_AcceptInverseOffers {
    // message fields
    inverse_offer_ids: ::protobuf::RepeatedField<super::mesos::OfferID>,
    filters: ::protobuf::SingularPtrField<super::mesos::Filters>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_AcceptInverseOffers {}

impl Call_AcceptInverseOffers {
    pub fn new() -> Call_AcceptInverseOffers {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_AcceptInverseOffers {
        static mut instance: ::protobuf::lazy::Lazy<Call_AcceptInverseOffers> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_AcceptInverseOffers,
        };
        unsafe {
            instance.get(Call_AcceptInverseOffers::new)
        }
    }

    // repeated .mesos.v1.OfferID inverse_offer_ids = 1;

    pub fn clear_inverse_offer_ids(&mut self) {
        self.inverse_offer_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_inverse_offer_ids(&mut self, v: ::protobuf::RepeatedField<super::mesos::OfferID>) {
        self.inverse_offer_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inverse_offer_ids(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.inverse_offer_ids
    }

    // Take field
    pub fn take_inverse_offer_ids(&mut self) -> ::protobuf::RepeatedField<super::mesos::OfferID> {
        ::std::mem::replace(&mut self.inverse_offer_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_inverse_offer_ids(&self) -> &[super::mesos::OfferID] {
        &self.inverse_offer_ids
    }

    fn get_inverse_offer_ids_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::OfferID> {
        &self.inverse_offer_ids
    }

    fn mut_inverse_offer_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.inverse_offer_ids
    }

    // optional .mesos.v1.Filters filters = 2;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    pub fn has_filters(&self) -> bool {
        self.filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: super::mesos::Filters) {
        self.filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filters(&mut self) -> &mut super::mesos::Filters {
        if self.filters.is_none() {
            self.filters.set_default();
        }
        self.filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_filters(&mut self) -> super::mesos::Filters {
        self.filters.take().unwrap_or_else(|| super::mesos::Filters::new())
    }

    pub fn get_filters(&self) -> &super::mesos::Filters {
        self.filters.as_ref().unwrap_or_else(|| super::mesos::Filters::default_instance())
    }

    fn get_filters_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::Filters> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::Filters> {
        &mut self.filters
    }
}

impl ::protobuf::Message for Call_AcceptInverseOffers {
    fn is_initialized(&self) -> bool {
        for v in &self.inverse_offer_ids {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inverse_offer_ids)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.inverse_offer_ids {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.filters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.inverse_offer_ids {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.filters.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_AcceptInverseOffers {
    fn new() -> Call_AcceptInverseOffers {
        Call_AcceptInverseOffers::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_AcceptInverseOffers>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OfferID>>(
                    "inverse_offer_ids",
                    Call_AcceptInverseOffers::get_inverse_offer_ids_for_reflect,
                    Call_AcceptInverseOffers::mut_inverse_offer_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Filters>>(
                    "filters",
                    Call_AcceptInverseOffers::get_filters_for_reflect,
                    Call_AcceptInverseOffers::mut_filters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_AcceptInverseOffers>(
                    "Call_AcceptInverseOffers",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_AcceptInverseOffers {
    fn clear(&mut self) {
        self.clear_inverse_offer_ids();
        self.clear_filters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_AcceptInverseOffers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_AcceptInverseOffers {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_DeclineInverseOffers {
    // message fields
    inverse_offer_ids: ::protobuf::RepeatedField<super::mesos::OfferID>,
    filters: ::protobuf::SingularPtrField<super::mesos::Filters>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_DeclineInverseOffers {}

impl Call_DeclineInverseOffers {
    pub fn new() -> Call_DeclineInverseOffers {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_DeclineInverseOffers {
        static mut instance: ::protobuf::lazy::Lazy<Call_DeclineInverseOffers> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_DeclineInverseOffers,
        };
        unsafe {
            instance.get(Call_DeclineInverseOffers::new)
        }
    }

    // repeated .mesos.v1.OfferID inverse_offer_ids = 1;

    pub fn clear_inverse_offer_ids(&mut self) {
        self.inverse_offer_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_inverse_offer_ids(&mut self, v: ::protobuf::RepeatedField<super::mesos::OfferID>) {
        self.inverse_offer_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inverse_offer_ids(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.inverse_offer_ids
    }

    // Take field
    pub fn take_inverse_offer_ids(&mut self) -> ::protobuf::RepeatedField<super::mesos::OfferID> {
        ::std::mem::replace(&mut self.inverse_offer_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_inverse_offer_ids(&self) -> &[super::mesos::OfferID] {
        &self.inverse_offer_ids
    }

    fn get_inverse_offer_ids_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::OfferID> {
        &self.inverse_offer_ids
    }

    fn mut_inverse_offer_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::OfferID> {
        &mut self.inverse_offer_ids
    }

    // optional .mesos.v1.Filters filters = 2;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    pub fn has_filters(&self) -> bool {
        self.filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: super::mesos::Filters) {
        self.filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filters(&mut self) -> &mut super::mesos::Filters {
        if self.filters.is_none() {
            self.filters.set_default();
        }
        self.filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_filters(&mut self) -> super::mesos::Filters {
        self.filters.take().unwrap_or_else(|| super::mesos::Filters::new())
    }

    pub fn get_filters(&self) -> &super::mesos::Filters {
        self.filters.as_ref().unwrap_or_else(|| super::mesos::Filters::default_instance())
    }

    fn get_filters_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::Filters> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::Filters> {
        &mut self.filters
    }
}

impl ::protobuf::Message for Call_DeclineInverseOffers {
    fn is_initialized(&self) -> bool {
        for v in &self.inverse_offer_ids {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inverse_offer_ids)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.inverse_offer_ids {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.filters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.inverse_offer_ids {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.filters.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_DeclineInverseOffers {
    fn new() -> Call_DeclineInverseOffers {
        Call_DeclineInverseOffers::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_DeclineInverseOffers>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OfferID>>(
                    "inverse_offer_ids",
                    Call_DeclineInverseOffers::get_inverse_offer_ids_for_reflect,
                    Call_DeclineInverseOffers::mut_inverse_offer_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Filters>>(
                    "filters",
                    Call_DeclineInverseOffers::get_filters_for_reflect,
                    Call_DeclineInverseOffers::mut_filters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_DeclineInverseOffers>(
                    "Call_DeclineInverseOffers",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_DeclineInverseOffers {
    fn clear(&mut self) {
        self.clear_inverse_offer_ids();
        self.clear_filters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_DeclineInverseOffers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_DeclineInverseOffers {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Revive {
    // message fields
    roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Revive {}

impl Call_Revive {
    pub fn new() -> Call_Revive {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Revive {
        static mut instance: ::protobuf::lazy::Lazy<Call_Revive> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Revive,
        };
        unsafe {
            instance.get(Call_Revive::new)
        }
    }

    // repeated string roles = 1;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }

    fn get_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.roles
    }

    fn mut_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }
}

impl ::protobuf::Message for Call_Revive {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.roles {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Revive {
    fn new() -> Call_Revive {
        Call_Revive::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Revive>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    Call_Revive::get_roles_for_reflect,
                    Call_Revive::mut_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Revive>(
                    "Call_Revive",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Revive {
    fn clear(&mut self) {
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Revive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Revive {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Kill {
    // message fields
    task_id: ::protobuf::SingularPtrField<super::mesos::TaskID>,
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    kill_policy: ::protobuf::SingularPtrField<super::mesos::KillPolicy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Kill {}

impl Call_Kill {
    pub fn new() -> Call_Kill {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Kill {
        static mut instance: ::protobuf::lazy::Lazy<Call_Kill> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Kill,
        };
        unsafe {
            instance.get(Call_Kill::new)
        }
    }

    // required .mesos.v1.TaskID task_id = 1;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: super::mesos::TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id(&mut self) -> &mut super::mesos::TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        }
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> super::mesos::TaskID {
        self.task_id.take().unwrap_or_else(|| super::mesos::TaskID::new())
    }

    pub fn get_task_id(&self) -> &super::mesos::TaskID {
        self.task_id.as_ref().unwrap_or_else(|| super::mesos::TaskID::default_instance())
    }

    fn get_task_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::TaskID> {
        &self.task_id
    }

    fn mut_task_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::TaskID> {
        &mut self.task_id
    }

    // optional .mesos.v1.AgentID agent_id = 2;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // optional .mesos.v1.KillPolicy kill_policy = 3;

    pub fn clear_kill_policy(&mut self) {
        self.kill_policy.clear();
    }

    pub fn has_kill_policy(&self) -> bool {
        self.kill_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kill_policy(&mut self, v: super::mesos::KillPolicy) {
        self.kill_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kill_policy(&mut self) -> &mut super::mesos::KillPolicy {
        if self.kill_policy.is_none() {
            self.kill_policy.set_default();
        }
        self.kill_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_kill_policy(&mut self) -> super::mesos::KillPolicy {
        self.kill_policy.take().unwrap_or_else(|| super::mesos::KillPolicy::new())
    }

    pub fn get_kill_policy(&self) -> &super::mesos::KillPolicy {
        self.kill_policy.as_ref().unwrap_or_else(|| super::mesos::KillPolicy::default_instance())
    }

    fn get_kill_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::KillPolicy> {
        &self.kill_policy
    }

    fn mut_kill_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::KillPolicy> {
        &mut self.kill_policy
    }
}

impl ::protobuf::Message for Call_Kill {
    fn is_initialized(&self) -> bool {
        if self.task_id.is_none() {
            return false;
        }
        for v in &self.task_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.kill_policy {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.task_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kill_policy)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.task_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.kill_policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.task_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.kill_policy.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Kill {
    fn new() -> Call_Kill {
        Call_Kill::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Kill>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::TaskID>>(
                    "task_id",
                    Call_Kill::get_task_id_for_reflect,
                    Call_Kill::mut_task_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_Kill::get_agent_id_for_reflect,
                    Call_Kill::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::KillPolicy>>(
                    "kill_policy",
                    Call_Kill::get_kill_policy_for_reflect,
                    Call_Kill::mut_kill_policy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Kill>(
                    "Call_Kill",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Kill {
    fn clear(&mut self) {
        self.clear_task_id();
        self.clear_agent_id();
        self.clear_kill_policy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Kill {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Kill {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Shutdown {
    // message fields
    executor_id: ::protobuf::SingularPtrField<super::mesos::ExecutorID>,
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Shutdown {}

impl Call_Shutdown {
    pub fn new() -> Call_Shutdown {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Shutdown {
        static mut instance: ::protobuf::lazy::Lazy<Call_Shutdown> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Shutdown,
        };
        unsafe {
            instance.get(Call_Shutdown::new)
        }
    }

    // required .mesos.v1.ExecutorID executor_id = 1;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: super::mesos::ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id(&mut self) -> &mut super::mesos::ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        }
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> super::mesos::ExecutorID {
        self.executor_id.take().unwrap_or_else(|| super::mesos::ExecutorID::new())
    }

    pub fn get_executor_id(&self) -> &super::mesos::ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| super::mesos::ExecutorID::default_instance())
    }

    fn get_executor_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &self.executor_id
    }

    fn mut_executor_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &mut self.executor_id
    }

    // required .mesos.v1.AgentID agent_id = 2;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }
}

impl ::protobuf::Message for Call_Shutdown {
    fn is_initialized(&self) -> bool {
        if self.executor_id.is_none() {
            return false;
        }
        if self.agent_id.is_none() {
            return false;
        }
        for v in &self.executor_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executor_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.executor_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.executor_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Shutdown {
    fn new() -> Call_Shutdown {
        Call_Shutdown::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Shutdown>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::ExecutorID>>(
                    "executor_id",
                    Call_Shutdown::get_executor_id_for_reflect,
                    Call_Shutdown::mut_executor_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_Shutdown::get_agent_id_for_reflect,
                    Call_Shutdown::mut_agent_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Shutdown>(
                    "Call_Shutdown",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Shutdown {
    fn clear(&mut self) {
        self.clear_executor_id();
        self.clear_agent_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Shutdown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Shutdown {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Acknowledge {
    // message fields
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    task_id: ::protobuf::SingularPtrField<super::mesos::TaskID>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Acknowledge {}

impl Call_Acknowledge {
    pub fn new() -> Call_Acknowledge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Acknowledge {
        static mut instance: ::protobuf::lazy::Lazy<Call_Acknowledge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Acknowledge,
        };
        unsafe {
            instance.get(Call_Acknowledge::new)
        }
    }

    // required .mesos.v1.AgentID agent_id = 1;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // required .mesos.v1.TaskID task_id = 2;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: super::mesos::TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id(&mut self) -> &mut super::mesos::TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        }
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> super::mesos::TaskID {
        self.task_id.take().unwrap_or_else(|| super::mesos::TaskID::new())
    }

    pub fn get_task_id(&self) -> &super::mesos::TaskID {
        self.task_id.as_ref().unwrap_or_else(|| super::mesos::TaskID::default_instance())
    }

    fn get_task_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::TaskID> {
        &self.task_id
    }

    fn mut_task_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::TaskID> {
        &mut self.task_id
    }

    // required bytes uuid = 3;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        }
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for Call_Acknowledge {
    fn is_initialized(&self) -> bool {
        if self.agent_id.is_none() {
            return false;
        }
        if self.task_id.is_none() {
            return false;
        }
        if self.uuid.is_none() {
            return false;
        }
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.task_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.task_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.task_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.task_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.uuid.as_ref() {
            os.write_bytes(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Acknowledge {
    fn new() -> Call_Acknowledge {
        Call_Acknowledge::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Acknowledge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_Acknowledge::get_agent_id_for_reflect,
                    Call_Acknowledge::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::TaskID>>(
                    "task_id",
                    Call_Acknowledge::get_task_id_for_reflect,
                    Call_Acknowledge::mut_task_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    Call_Acknowledge::get_uuid_for_reflect,
                    Call_Acknowledge::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Acknowledge>(
                    "Call_Acknowledge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Acknowledge {
    fn clear(&mut self) {
        self.clear_agent_id();
        self.clear_task_id();
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Acknowledge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Acknowledge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_AcknowledgeOperationStatus {
    // message fields
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    resource_provider_id: ::protobuf::SingularPtrField<super::mesos::ResourceProviderID>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    operation_id: ::protobuf::SingularPtrField<super::mesos::OperationID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_AcknowledgeOperationStatus {}

impl Call_AcknowledgeOperationStatus {
    pub fn new() -> Call_AcknowledgeOperationStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_AcknowledgeOperationStatus {
        static mut instance: ::protobuf::lazy::Lazy<Call_AcknowledgeOperationStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_AcknowledgeOperationStatus,
        };
        unsafe {
            instance.get(Call_AcknowledgeOperationStatus::new)
        }
    }

    // optional .mesos.v1.AgentID agent_id = 1;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // optional .mesos.v1.ResourceProviderID resource_provider_id = 2;

    pub fn clear_resource_provider_id(&mut self) {
        self.resource_provider_id.clear();
    }

    pub fn has_resource_provider_id(&self) -> bool {
        self.resource_provider_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource_provider_id(&mut self, v: super::mesos::ResourceProviderID) {
        self.resource_provider_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource_provider_id(&mut self) -> &mut super::mesos::ResourceProviderID {
        if self.resource_provider_id.is_none() {
            self.resource_provider_id.set_default();
        }
        self.resource_provider_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource_provider_id(&mut self) -> super::mesos::ResourceProviderID {
        self.resource_provider_id.take().unwrap_or_else(|| super::mesos::ResourceProviderID::new())
    }

    pub fn get_resource_provider_id(&self) -> &super::mesos::ResourceProviderID {
        self.resource_provider_id.as_ref().unwrap_or_else(|| super::mesos::ResourceProviderID::default_instance())
    }

    fn get_resource_provider_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::ResourceProviderID> {
        &self.resource_provider_id
    }

    fn mut_resource_provider_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::ResourceProviderID> {
        &mut self.resource_provider_id
    }

    // required bytes uuid = 3;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        }
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.uuid
    }

    // required .mesos.v1.OperationID operation_id = 4;

    pub fn clear_operation_id(&mut self) {
        self.operation_id.clear();
    }

    pub fn has_operation_id(&self) -> bool {
        self.operation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operation_id(&mut self, v: super::mesos::OperationID) {
        self.operation_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation_id(&mut self) -> &mut super::mesos::OperationID {
        if self.operation_id.is_none() {
            self.operation_id.set_default();
        }
        self.operation_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_operation_id(&mut self) -> super::mesos::OperationID {
        self.operation_id.take().unwrap_or_else(|| super::mesos::OperationID::new())
    }

    pub fn get_operation_id(&self) -> &super::mesos::OperationID {
        self.operation_id.as_ref().unwrap_or_else(|| super::mesos::OperationID::default_instance())
    }

    fn get_operation_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::OperationID> {
        &self.operation_id
    }

    fn mut_operation_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::OperationID> {
        &mut self.operation_id
    }
}

impl ::protobuf::Message for Call_AcknowledgeOperationStatus {
    fn is_initialized(&self) -> bool {
        if self.uuid.is_none() {
            return false;
        }
        if self.operation_id.is_none() {
            return false;
        }
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource_provider_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.operation_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resource_provider_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.operation_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.resource_provider_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.operation_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.resource_provider_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.uuid.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.operation_id.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_AcknowledgeOperationStatus {
    fn new() -> Call_AcknowledgeOperationStatus {
        Call_AcknowledgeOperationStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_AcknowledgeOperationStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_AcknowledgeOperationStatus::get_agent_id_for_reflect,
                    Call_AcknowledgeOperationStatus::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::ResourceProviderID>>(
                    "resource_provider_id",
                    Call_AcknowledgeOperationStatus::get_resource_provider_id_for_reflect,
                    Call_AcknowledgeOperationStatus::mut_resource_provider_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    Call_AcknowledgeOperationStatus::get_uuid_for_reflect,
                    Call_AcknowledgeOperationStatus::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OperationID>>(
                    "operation_id",
                    Call_AcknowledgeOperationStatus::get_operation_id_for_reflect,
                    Call_AcknowledgeOperationStatus::mut_operation_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_AcknowledgeOperationStatus>(
                    "Call_AcknowledgeOperationStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_AcknowledgeOperationStatus {
    fn clear(&mut self) {
        self.clear_agent_id();
        self.clear_resource_provider_id();
        self.clear_uuid();
        self.clear_operation_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_AcknowledgeOperationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_AcknowledgeOperationStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Reconcile {
    // message fields
    tasks: ::protobuf::RepeatedField<Call_Reconcile_Task>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Reconcile {}

impl Call_Reconcile {
    pub fn new() -> Call_Reconcile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Reconcile {
        static mut instance: ::protobuf::lazy::Lazy<Call_Reconcile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Reconcile,
        };
        unsafe {
            instance.get(Call_Reconcile::new)
        }
    }

    // repeated .mesos.v1.scheduler.Call.Reconcile.Task tasks = 1;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<Call_Reconcile_Task>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks(&mut self) -> &mut ::protobuf::RepeatedField<Call_Reconcile_Task> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<Call_Reconcile_Task> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks(&self) -> &[Call_Reconcile_Task] {
        &self.tasks
    }

    fn get_tasks_for_reflect(&self) -> &::protobuf::RepeatedField<Call_Reconcile_Task> {
        &self.tasks
    }

    fn mut_tasks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Call_Reconcile_Task> {
        &mut self.tasks
    }
}

impl ::protobuf::Message for Call_Reconcile {
    fn is_initialized(&self) -> bool {
        for v in &self.tasks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.tasks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tasks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Reconcile {
    fn new() -> Call_Reconcile {
        Call_Reconcile::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Reconcile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_Reconcile_Task>>(
                    "tasks",
                    Call_Reconcile::get_tasks_for_reflect,
                    Call_Reconcile::mut_tasks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Reconcile>(
                    "Call_Reconcile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Reconcile {
    fn clear(&mut self) {
        self.clear_tasks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Reconcile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Reconcile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Reconcile_Task {
    // message fields
    task_id: ::protobuf::SingularPtrField<super::mesos::TaskID>,
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Reconcile_Task {}

impl Call_Reconcile_Task {
    pub fn new() -> Call_Reconcile_Task {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Reconcile_Task {
        static mut instance: ::protobuf::lazy::Lazy<Call_Reconcile_Task> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Reconcile_Task,
        };
        unsafe {
            instance.get(Call_Reconcile_Task::new)
        }
    }

    // required .mesos.v1.TaskID task_id = 1;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: super::mesos::TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id(&mut self) -> &mut super::mesos::TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        }
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> super::mesos::TaskID {
        self.task_id.take().unwrap_or_else(|| super::mesos::TaskID::new())
    }

    pub fn get_task_id(&self) -> &super::mesos::TaskID {
        self.task_id.as_ref().unwrap_or_else(|| super::mesos::TaskID::default_instance())
    }

    fn get_task_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::TaskID> {
        &self.task_id
    }

    fn mut_task_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::TaskID> {
        &mut self.task_id
    }

    // optional .mesos.v1.AgentID agent_id = 2;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }
}

impl ::protobuf::Message for Call_Reconcile_Task {
    fn is_initialized(&self) -> bool {
        if self.task_id.is_none() {
            return false;
        }
        for v in &self.task_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.task_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.task_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.task_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Reconcile_Task {
    fn new() -> Call_Reconcile_Task {
        Call_Reconcile_Task::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Reconcile_Task>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::TaskID>>(
                    "task_id",
                    Call_Reconcile_Task::get_task_id_for_reflect,
                    Call_Reconcile_Task::mut_task_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_Reconcile_Task::get_agent_id_for_reflect,
                    Call_Reconcile_Task::mut_agent_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Reconcile_Task>(
                    "Call_Reconcile_Task",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Reconcile_Task {
    fn clear(&mut self) {
        self.clear_task_id();
        self.clear_agent_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Reconcile_Task {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Reconcile_Task {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_ReconcileOperations {
    // message fields
    operations: ::protobuf::RepeatedField<Call_ReconcileOperations_Operation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_ReconcileOperations {}

impl Call_ReconcileOperations {
    pub fn new() -> Call_ReconcileOperations {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_ReconcileOperations {
        static mut instance: ::protobuf::lazy::Lazy<Call_ReconcileOperations> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_ReconcileOperations,
        };
        unsafe {
            instance.get(Call_ReconcileOperations::new)
        }
    }

    // repeated .mesos.v1.scheduler.Call.ReconcileOperations.Operation operations = 1;

    pub fn clear_operations(&mut self) {
        self.operations.clear();
    }

    // Param is passed by value, moved
    pub fn set_operations(&mut self, v: ::protobuf::RepeatedField<Call_ReconcileOperations_Operation>) {
        self.operations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_operations(&mut self) -> &mut ::protobuf::RepeatedField<Call_ReconcileOperations_Operation> {
        &mut self.operations
    }

    // Take field
    pub fn take_operations(&mut self) -> ::protobuf::RepeatedField<Call_ReconcileOperations_Operation> {
        ::std::mem::replace(&mut self.operations, ::protobuf::RepeatedField::new())
    }

    pub fn get_operations(&self) -> &[Call_ReconcileOperations_Operation] {
        &self.operations
    }

    fn get_operations_for_reflect(&self) -> &::protobuf::RepeatedField<Call_ReconcileOperations_Operation> {
        &self.operations
    }

    fn mut_operations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Call_ReconcileOperations_Operation> {
        &mut self.operations
    }
}

impl ::protobuf::Message for Call_ReconcileOperations {
    fn is_initialized(&self) -> bool {
        for v in &self.operations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.operations)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.operations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.operations {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_ReconcileOperations {
    fn new() -> Call_ReconcileOperations {
        Call_ReconcileOperations::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_ReconcileOperations>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Call_ReconcileOperations_Operation>>(
                    "operations",
                    Call_ReconcileOperations::get_operations_for_reflect,
                    Call_ReconcileOperations::mut_operations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_ReconcileOperations>(
                    "Call_ReconcileOperations",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_ReconcileOperations {
    fn clear(&mut self) {
        self.clear_operations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_ReconcileOperations {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_ReconcileOperations {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_ReconcileOperations_Operation {
    // message fields
    operation_id: ::protobuf::SingularPtrField<super::mesos::OperationID>,
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    resource_provider_id: ::protobuf::SingularPtrField<super::mesos::ResourceProviderID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_ReconcileOperations_Operation {}

impl Call_ReconcileOperations_Operation {
    pub fn new() -> Call_ReconcileOperations_Operation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_ReconcileOperations_Operation {
        static mut instance: ::protobuf::lazy::Lazy<Call_ReconcileOperations_Operation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_ReconcileOperations_Operation,
        };
        unsafe {
            instance.get(Call_ReconcileOperations_Operation::new)
        }
    }

    // required .mesos.v1.OperationID operation_id = 1;

    pub fn clear_operation_id(&mut self) {
        self.operation_id.clear();
    }

    pub fn has_operation_id(&self) -> bool {
        self.operation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operation_id(&mut self, v: super::mesos::OperationID) {
        self.operation_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation_id(&mut self) -> &mut super::mesos::OperationID {
        if self.operation_id.is_none() {
            self.operation_id.set_default();
        }
        self.operation_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_operation_id(&mut self) -> super::mesos::OperationID {
        self.operation_id.take().unwrap_or_else(|| super::mesos::OperationID::new())
    }

    pub fn get_operation_id(&self) -> &super::mesos::OperationID {
        self.operation_id.as_ref().unwrap_or_else(|| super::mesos::OperationID::default_instance())
    }

    fn get_operation_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::OperationID> {
        &self.operation_id
    }

    fn mut_operation_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::OperationID> {
        &mut self.operation_id
    }

    // optional .mesos.v1.AgentID agent_id = 2;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // optional .mesos.v1.ResourceProviderID resource_provider_id = 3;

    pub fn clear_resource_provider_id(&mut self) {
        self.resource_provider_id.clear();
    }

    pub fn has_resource_provider_id(&self) -> bool {
        self.resource_provider_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource_provider_id(&mut self, v: super::mesos::ResourceProviderID) {
        self.resource_provider_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource_provider_id(&mut self) -> &mut super::mesos::ResourceProviderID {
        if self.resource_provider_id.is_none() {
            self.resource_provider_id.set_default();
        }
        self.resource_provider_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource_provider_id(&mut self) -> super::mesos::ResourceProviderID {
        self.resource_provider_id.take().unwrap_or_else(|| super::mesos::ResourceProviderID::new())
    }

    pub fn get_resource_provider_id(&self) -> &super::mesos::ResourceProviderID {
        self.resource_provider_id.as_ref().unwrap_or_else(|| super::mesos::ResourceProviderID::default_instance())
    }

    fn get_resource_provider_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::ResourceProviderID> {
        &self.resource_provider_id
    }

    fn mut_resource_provider_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::ResourceProviderID> {
        &mut self.resource_provider_id
    }
}

impl ::protobuf::Message for Call_ReconcileOperations_Operation {
    fn is_initialized(&self) -> bool {
        if self.operation_id.is_none() {
            return false;
        }
        for v in &self.operation_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource_provider_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.operation_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resource_provider_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.operation_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.resource_provider_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.operation_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.resource_provider_id.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_ReconcileOperations_Operation {
    fn new() -> Call_ReconcileOperations_Operation {
        Call_ReconcileOperations_Operation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_ReconcileOperations_Operation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::OperationID>>(
                    "operation_id",
                    Call_ReconcileOperations_Operation::get_operation_id_for_reflect,
                    Call_ReconcileOperations_Operation::mut_operation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_ReconcileOperations_Operation::get_agent_id_for_reflect,
                    Call_ReconcileOperations_Operation::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::ResourceProviderID>>(
                    "resource_provider_id",
                    Call_ReconcileOperations_Operation::get_resource_provider_id_for_reflect,
                    Call_ReconcileOperations_Operation::mut_resource_provider_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_ReconcileOperations_Operation>(
                    "Call_ReconcileOperations_Operation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_ReconcileOperations_Operation {
    fn clear(&mut self) {
        self.clear_operation_id();
        self.clear_agent_id();
        self.clear_resource_provider_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_ReconcileOperations_Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_ReconcileOperations_Operation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Message {
    // message fields
    agent_id: ::protobuf::SingularPtrField<super::mesos::AgentID>,
    executor_id: ::protobuf::SingularPtrField<super::mesos::ExecutorID>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Message {}

impl Call_Message {
    pub fn new() -> Call_Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Message {
        static mut instance: ::protobuf::lazy::Lazy<Call_Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Message,
        };
        unsafe {
            instance.get(Call_Message::new)
        }
    }

    // required .mesos.v1.AgentID agent_id = 1;

    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    pub fn has_agent_id(&self) -> bool {
        self.agent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: super::mesos::AgentID) {
        self.agent_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut super::mesos::AgentID {
        if self.agent_id.is_none() {
            self.agent_id.set_default();
        }
        self.agent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_agent_id(&mut self) -> super::mesos::AgentID {
        self.agent_id.take().unwrap_or_else(|| super::mesos::AgentID::new())
    }

    pub fn get_agent_id(&self) -> &super::mesos::AgentID {
        self.agent_id.as_ref().unwrap_or_else(|| super::mesos::AgentID::default_instance())
    }

    fn get_agent_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::AgentID> {
        &self.agent_id
    }

    fn mut_agent_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::AgentID> {
        &mut self.agent_id
    }

    // required .mesos.v1.ExecutorID executor_id = 2;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: super::mesos::ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id(&mut self) -> &mut super::mesos::ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        }
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> super::mesos::ExecutorID {
        self.executor_id.take().unwrap_or_else(|| super::mesos::ExecutorID::new())
    }

    pub fn get_executor_id(&self) -> &super::mesos::ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| super::mesos::ExecutorID::default_instance())
    }

    fn get_executor_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &self.executor_id
    }

    fn mut_executor_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::mesos::ExecutorID> {
        &mut self.executor_id
    }

    // required bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for Call_Message {
    fn is_initialized(&self) -> bool {
        if self.agent_id.is_none() {
            return false;
        }
        if self.executor_id.is_none() {
            return false;
        }
        if self.data.is_none() {
            return false;
        }
        for v in &self.agent_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.executor_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executor_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.agent_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.executor_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.agent_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.executor_id.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Message {
    fn new() -> Call_Message {
        Call_Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::AgentID>>(
                    "agent_id",
                    Call_Message::get_agent_id_for_reflect,
                    Call_Message::mut_agent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::ExecutorID>>(
                    "executor_id",
                    Call_Message::get_executor_id_for_reflect,
                    Call_Message::mut_executor_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Call_Message::get_data_for_reflect,
                    Call_Message::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Message>(
                    "Call_Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Message {
    fn clear(&mut self) {
        self.clear_agent_id();
        self.clear_executor_id();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Request {
    // message fields
    requests: ::protobuf::RepeatedField<super::mesos::Request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Request {}

impl Call_Request {
    pub fn new() -> Call_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Request {
        static mut instance: ::protobuf::lazy::Lazy<Call_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Request,
        };
        unsafe {
            instance.get(Call_Request::new)
        }
    }

    // repeated .mesos.v1.Request requests = 1;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<super::mesos::Request>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::Request> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<super::mesos::Request> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests(&self) -> &[super::mesos::Request] {
        &self.requests
    }

    fn get_requests_for_reflect(&self) -> &::protobuf::RepeatedField<super::mesos::Request> {
        &self.requests
    }

    fn mut_requests_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::mesos::Request> {
        &mut self.requests
    }
}

impl ::protobuf::Message for Call_Request {
    fn is_initialized(&self) -> bool {
        for v in &self.requests {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.requests {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Request {
    fn new() -> Call_Request {
        Call_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::mesos::Request>>(
                    "requests",
                    Call_Request::get_requests_for_reflect,
                    Call_Request::mut_requests_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Request>(
                    "Call_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Request {
    fn clear(&mut self) {
        self.clear_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call_Suppress {
    // message fields
    roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call_Suppress {}

impl Call_Suppress {
    pub fn new() -> Call_Suppress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call_Suppress {
        static mut instance: ::protobuf::lazy::Lazy<Call_Suppress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call_Suppress,
        };
        unsafe {
            instance.get(Call_Suppress::new)
        }
    }

    // repeated string roles = 1;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }

    fn get_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.roles
    }

    fn mut_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }
}

impl ::protobuf::Message for Call_Suppress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.roles {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Call_Suppress {
    fn new() -> Call_Suppress {
        Call_Suppress::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call_Suppress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    Call_Suppress::get_roles_for_reflect,
                    Call_Suppress::mut_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call_Suppress>(
                    "Call_Suppress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call_Suppress {
    fn clear(&mut self) {
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call_Suppress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call_Suppress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Call_Type {
    UNKNOWN = 0,
    SUBSCRIBE = 1,
    TEARDOWN = 2,
    ACCEPT = 3,
    DECLINE = 4,
    ACCEPT_INVERSE_OFFERS = 13,
    DECLINE_INVERSE_OFFERS = 14,
    REVIVE = 5,
    KILL = 6,
    SHUTDOWN = 7,
    ACKNOWLEDGE = 8,
    ACKNOWLEDGE_OPERATION_STATUS = 15,
    RECONCILE = 9,
    RECONCILE_OPERATIONS = 16,
    MESSAGE = 10,
    REQUEST = 11,
    SUPPRESS = 12,
}

impl ::protobuf::ProtobufEnum for Call_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Call_Type> {
        match value {
            0 => ::std::option::Option::Some(Call_Type::UNKNOWN),
            1 => ::std::option::Option::Some(Call_Type::SUBSCRIBE),
            2 => ::std::option::Option::Some(Call_Type::TEARDOWN),
            3 => ::std::option::Option::Some(Call_Type::ACCEPT),
            4 => ::std::option::Option::Some(Call_Type::DECLINE),
            13 => ::std::option::Option::Some(Call_Type::ACCEPT_INVERSE_OFFERS),
            14 => ::std::option::Option::Some(Call_Type::DECLINE_INVERSE_OFFERS),
            5 => ::std::option::Option::Some(Call_Type::REVIVE),
            6 => ::std::option::Option::Some(Call_Type::KILL),
            7 => ::std::option::Option::Some(Call_Type::SHUTDOWN),
            8 => ::std::option::Option::Some(Call_Type::ACKNOWLEDGE),
            15 => ::std::option::Option::Some(Call_Type::ACKNOWLEDGE_OPERATION_STATUS),
            9 => ::std::option::Option::Some(Call_Type::RECONCILE),
            16 => ::std::option::Option::Some(Call_Type::RECONCILE_OPERATIONS),
            10 => ::std::option::Option::Some(Call_Type::MESSAGE),
            11 => ::std::option::Option::Some(Call_Type::REQUEST),
            12 => ::std::option::Option::Some(Call_Type::SUPPRESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Call_Type] = &[
            Call_Type::UNKNOWN,
            Call_Type::SUBSCRIBE,
            Call_Type::TEARDOWN,
            Call_Type::ACCEPT,
            Call_Type::DECLINE,
            Call_Type::ACCEPT_INVERSE_OFFERS,
            Call_Type::DECLINE_INVERSE_OFFERS,
            Call_Type::REVIVE,
            Call_Type::KILL,
            Call_Type::SHUTDOWN,
            Call_Type::ACKNOWLEDGE,
            Call_Type::ACKNOWLEDGE_OPERATION_STATUS,
            Call_Type::RECONCILE,
            Call_Type::RECONCILE_OPERATIONS,
            Call_Type::MESSAGE,
            Call_Type::REQUEST,
            Call_Type::SUPPRESS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Call_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Call_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Call_Type {
}

impl ::protobuf::reflect::ProtobufValue for Call_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"mesos/v1/scheduler/scheduler.proto\x12\x12mesos.v1.scheduler\x1a\x14\
    mesos/v1/mesos.proto\"\xcd\x0e\n\x05Event\x122\n\x04type\x18\x01\x20\x01\
    (\x0e2\x1e.mesos.v1.scheduler.Event.TypeR\x04type\x12D\n\nsubscribed\x18\
    \x02\x20\x01(\x0b2$.mesos.v1.scheduler.Event.SubscribedR\nsubscribed\x12\
    8\n\x06offers\x18\x03\x20\x01(\x0b2\x20.mesos.v1.scheduler.Event.OffersR\
    \x06offers\x12N\n\x0einverse_offers\x18\t\x20\x01(\x0b2'.mesos.v1.schedu\
    ler.Event.InverseOffersR\rinverseOffers\x12;\n\x07rescind\x18\x04\x20\
    \x01(\x0b2!.mesos.v1.scheduler.Event.RescindR\x07rescind\x12a\n\x15resci\
    nd_inverse_offer\x18\n\x20\x01(\x0b2-.mesos.v1.scheduler.Event.RescindIn\
    verseOfferR\x13rescindInverseOffer\x128\n\x06update\x18\x05\x20\x01(\x0b\
    2\x20.mesos.v1.scheduler.Event.UpdateR\x06update\x12g\n\x17update_operat\
    ion_status\x18\x0b\x20\x01(\x0b2/.mesos.v1.scheduler.Event.UpdateOperati\
    onStatusR\x15updateOperationStatus\x12;\n\x07message\x18\x06\x20\x01(\
    \x0b2!.mesos.v1.scheduler.Event.MessageR\x07message\x12;\n\x07failure\
    \x18\x07\x20\x01(\x0b2!.mesos.v1.scheduler.Event.FailureR\x07failure\x12\
    5\n\x05error\x18\x08\x20\x01(\x0b2\x1f.mesos.v1.scheduler.Event.ErrorR\
    \x05error\x1a\xbb\x01\n\nSubscribed\x128\n\x0cframework_id\x18\x01\x20\
    \x02(\x0b2\x15.mesos.v1.FrameworkIDR\x0bframeworkId\x12<\n\x1aheartbeat_\
    interval_seconds\x18\x02\x20\x01(\x01R\x18heartbeatIntervalSeconds\x125\
    \n\x0bmaster_info\x18\x03\x20\x01(\x0b2\x14.mesos.v1.MasterInfoR\nmaster\
    Info\x1a1\n\x06Offers\x12'\n\x06offers\x18\x01\x20\x03(\x0b2\x0f.mesos.v\
    1.OfferR\x06offers\x1aN\n\rInverseOffers\x12=\n\x0einverse_offers\x18\
    \x01\x20\x03(\x0b2\x16.mesos.v1.InverseOfferR\rinverseOffers\x1a7\n\x07R\
    escind\x12,\n\x08offer_id\x18\x01\x20\x02(\x0b2\x11.mesos.v1.OfferIDR\
    \x07offerId\x1aR\n\x13RescindInverseOffer\x12;\n\x10inverse_offer_id\x18\
    \x01\x20\x02(\x0b2\x11.mesos.v1.OfferIDR\x0einverseOfferId\x1a6\n\x06Upd\
    ate\x12,\n\x06status\x18\x01\x20\x02(\x0b2\x14.mesos.v1.TaskStatusR\x06s\
    tatus\x1aJ\n\x15UpdateOperationStatus\x121\n\x06status\x18\x01\x20\x02(\
    \x0b2\x19.mesos.v1.OperationStatusR\x06status\x1a\x82\x01\n\x07Message\
    \x12,\n\x08agent_id\x18\x01\x20\x02(\x0b2\x11.mesos.v1.AgentIDR\x07agent\
    Id\x125\n\x0bexecutor_id\x18\x02\x20\x02(\x0b2\x14.mesos.v1.ExecutorIDR\
    \nexecutorId\x12\x12\n\x04data\x18\x03\x20\x02(\x0cR\x04data\x1a\x86\x01\
    \n\x07Failure\x12,\n\x08agent_id\x18\x01\x20\x01(\x0b2\x11.mesos.v1.Agen\
    tIDR\x07agentId\x125\n\x0bexecutor_id\x18\x02\x20\x01(\x0b2\x14.mesos.v1\
    .ExecutorIDR\nexecutorId\x12\x16\n\x06status\x18\x03\x20\x01(\x05R\x06st\
    atus\x1a!\n\x05Error\x12\x18\n\x07message\x18\x01\x20\x02(\tR\x07message\
    \"\xc8\x01\n\x04Type\x12\x0b\n\x07UNKNOWN\x10\0\x12\x0e\n\nSUBSCRIBED\
    \x10\x01\x12\n\n\x06OFFERS\x10\x02\x12\x12\n\x0eINVERSE_OFFERS\x10\t\x12\
    \x0b\n\x07RESCIND\x10\x03\x12\x19\n\x15RESCIND_INVERSE_OFFER\x10\n\x12\n\
    \n\x06UPDATE\x10\x04\x12\x1b\n\x17UPDATE_OPERATION_STATUS\x10\x0b\x12\
    \x0b\n\x07MESSAGE\x10\x05\x12\x0b\n\x07FAILURE\x10\x06\x12\t\n\x05ERROR\
    \x10\x07\x12\r\n\tHEARTBEAT\x10\x08\"\xff\x01\n\x08Response\x12c\n\x14re\
    concile_operations\x18\x01\x20\x01(\x0b20.mesos.v1.scheduler.Response.Re\
    concileOperationsR\x13reconcileOperations\x1a_\n\x13ReconcileOperations\
    \x12H\n\x12operation_statuses\x18\x01\x20\x03(\x0b2\x19.mesos.v1.Operati\
    onStatusR\x11operationStatuses\"-\n\x04Type\x12\x0b\n\x07UNKNOWN\x10\0\
    \x12\x18\n\x14RECONCILE_OPERATIONS\x10\x01\"\xc2\x1b\n\x04Call\x128\n\
    \x0cframework_id\x18\x01\x20\x01(\x0b2\x15.mesos.v1.FrameworkIDR\x0bfram\
    eworkId\x121\n\x04type\x18\x02\x20\x01(\x0e2\x1d.mesos.v1.scheduler.Call\
    .TypeR\x04type\x12@\n\tsubscribe\x18\x03\x20\x01(\x0b2\".mesos.v1.schedu\
    ler.Call.SubscribeR\tsubscribe\x127\n\x06accept\x18\x04\x20\x01(\x0b2\
    \x1f.mesos.v1.scheduler.Call.AcceptR\x06accept\x12:\n\x07decline\x18\x05\
    \x20\x01(\x0b2\x20.mesos.v1.scheduler.Call.DeclineR\x07decline\x12`\n\
    \x15accept_inverse_offers\x18\r\x20\x01(\x0b2,.mesos.v1.scheduler.Call.A\
    cceptInverseOffersR\x13acceptInverseOffers\x12c\n\x16decline_inverse_off\
    ers\x18\x0e\x20\x01(\x0b2-.mesos.v1.scheduler.Call.DeclineInverseOffersR\
    \x14declineInverseOffers\x127\n\x06revive\x18\x0f\x20\x01(\x0b2\x1f.meso\
    s.v1.scheduler.Call.ReviveR\x06revive\x121\n\x04kill\x18\x06\x20\x01(\
    \x0b2\x1d.mesos.v1.scheduler.Call.KillR\x04kill\x12=\n\x08shutdown\x18\
    \x07\x20\x01(\x0b2!.mesos.v1.scheduler.Call.ShutdownR\x08shutdown\x12F\n\
    \x0backnowledge\x18\x08\x20\x01(\x0b2$.mesos.v1.scheduler.Call.Acknowled\
    geR\x0backnowledge\x12u\n\x1cacknowledge_operation_status\x18\x11\x20\
    \x01(\x0b23.mesos.v1.scheduler.Call.AcknowledgeOperationStatusR\x1aackno\
    wledgeOperationStatus\x12@\n\treconcile\x18\t\x20\x01(\x0b2\".mesos.v1.s\
    cheduler.Call.ReconcileR\treconcile\x12_\n\x14reconcile_operations\x18\
    \x12\x20\x01(\x0b2,.mesos.v1.scheduler.Call.ReconcileOperationsR\x13reco\
    ncileOperations\x12:\n\x07message\x18\n\x20\x01(\x0b2\x20.mesos.v1.sched\
    uler.Call.MessageR\x07message\x12:\n\x07request\x18\x0b\x20\x01(\x0b2\
    \x20.mesos.v1.scheduler.Call.RequestR\x07request\x12=\n\x08suppress\x18\
    \x10\x20\x01(\x0b2!.mesos.v1.scheduler.Call.SuppressR\x08suppress\x1av\n\
    \tSubscribe\x12>\n\x0eframework_info\x18\x01\x20\x02(\x0b2\x17.mesos.v1.\
    FrameworkInfoR\rframeworkInfo\x12)\n\x10suppressed_roles\x18\x02\x20\x03\
    (\tR\x0fsuppressedRoles\x1a\xa0\x01\n\x06Accept\x12.\n\toffer_ids\x18\
    \x01\x20\x03(\x0b2\x11.mesos.v1.OfferIDR\x08offerIds\x129\n\noperations\
    \x18\x02\x20\x03(\x0b2\x19.mesos.v1.Offer.OperationR\noperations\x12+\n\
    \x07filters\x18\x03\x20\x01(\x0b2\x11.mesos.v1.FiltersR\x07filters\x1af\
    \n\x07Decline\x12.\n\toffer_ids\x18\x01\x20\x03(\x0b2\x11.mesos.v1.Offer\
    IDR\x08offerIds\x12+\n\x07filters\x18\x02\x20\x01(\x0b2\x11.mesos.v1.Fil\
    tersR\x07filters\x1a\x81\x01\n\x13AcceptInverseOffers\x12=\n\x11inverse_\
    offer_ids\x18\x01\x20\x03(\x0b2\x11.mesos.v1.OfferIDR\x0finverseOfferIds\
    \x12+\n\x07filters\x18\x02\x20\x01(\x0b2\x11.mesos.v1.FiltersR\x07filter\
    s\x1a\x82\x01\n\x14DeclineInverseOffers\x12=\n\x11inverse_offer_ids\x18\
    \x01\x20\x03(\x0b2\x11.mesos.v1.OfferIDR\x0finverseOfferIds\x12+\n\x07fi\
    lters\x18\x02\x20\x01(\x0b2\x11.mesos.v1.FiltersR\x07filters\x1a\x1e\n\
    \x06Revive\x12\x14\n\x05roles\x18\x01\x20\x03(\tR\x05roles\x1a\x96\x01\n\
    \x04Kill\x12)\n\x07task_id\x18\x01\x20\x02(\x0b2\x10.mesos.v1.TaskIDR\
    \x06taskId\x12,\n\x08agent_id\x18\x02\x20\x01(\x0b2\x11.mesos.v1.AgentID\
    R\x07agentId\x125\n\x0bkill_policy\x18\x03\x20\x01(\x0b2\x14.mesos.v1.Ki\
    llPolicyR\nkillPolicy\x1ao\n\x08Shutdown\x125\n\x0bexecutor_id\x18\x01\
    \x20\x02(\x0b2\x14.mesos.v1.ExecutorIDR\nexecutorId\x12,\n\x08agent_id\
    \x18\x02\x20\x02(\x0b2\x11.mesos.v1.AgentIDR\x07agentId\x1az\n\x0bAcknow\
    ledge\x12,\n\x08agent_id\x18\x01\x20\x02(\x0b2\x11.mesos.v1.AgentIDR\x07\
    agentId\x12)\n\x07task_id\x18\x02\x20\x02(\x0b2\x10.mesos.v1.TaskIDR\x06\
    taskId\x12\x12\n\x04uuid\x18\x03\x20\x02(\x0cR\x04uuid\x1a\xe8\x01\n\x1a\
    AcknowledgeOperationStatus\x12,\n\x08agent_id\x18\x01\x20\x01(\x0b2\x11.\
    mesos.v1.AgentIDR\x07agentId\x12N\n\x14resource_provider_id\x18\x02\x20\
    \x01(\x0b2\x1c.mesos.v1.ResourceProviderIDR\x12resourceProviderId\x12\
    \x12\n\x04uuid\x18\x03\x20\x02(\x0cR\x04uuid\x128\n\x0coperation_id\x18\
    \x04\x20\x02(\x0b2\x15.mesos.v1.OperationIDR\x0boperationId\x1a\xab\x01\
    \n\tReconcile\x12=\n\x05tasks\x18\x01\x20\x03(\x0b2'.mesos.v1.scheduler.\
    Call.Reconcile.TaskR\x05tasks\x1a_\n\x04Task\x12)\n\x07task_id\x18\x01\
    \x20\x02(\x0b2\x10.mesos.v1.TaskIDR\x06taskId\x12,\n\x08agent_id\x18\x02\
    \x20\x01(\x0b2\x11.mesos.v1.AgentIDR\x07agentId\x1a\xb3\x02\n\x13Reconci\
    leOperations\x12V\n\noperations\x18\x01\x20\x03(\x0b26.mesos.v1.schedule\
    r.Call.ReconcileOperations.OperationR\noperations\x1a\xc3\x01\n\tOperati\
    on\x128\n\x0coperation_id\x18\x01\x20\x02(\x0b2\x15.mesos.v1.OperationID\
    R\x0boperationId\x12,\n\x08agent_id\x18\x02\x20\x01(\x0b2\x11.mesos.v1.A\
    gentIDR\x07agentId\x12N\n\x14resource_provider_id\x18\x03\x20\x01(\x0b2\
    \x1c.mesos.v1.ResourceProviderIDR\x12resourceProviderId\x1a\x82\x01\n\
    \x07Message\x12,\n\x08agent_id\x18\x01\x20\x02(\x0b2\x11.mesos.v1.AgentI\
    DR\x07agentId\x125\n\x0bexecutor_id\x18\x02\x20\x02(\x0b2\x14.mesos.v1.E\
    xecutorIDR\nexecutorId\x12\x12\n\x04data\x18\x03\x20\x02(\x0cR\x04data\
    \x1a8\n\x07Request\x12-\n\x08requests\x18\x01\x20\x03(\x0b2\x11.mesos.v1\
    .RequestR\x08requests\x1a\x20\n\x08Suppress\x12\x14\n\x05roles\x18\x01\
    \x20\x03(\tR\x05roles\"\xa8\x02\n\x04Type\x12\x0b\n\x07UNKNOWN\x10\0\x12\
    \r\n\tSUBSCRIBE\x10\x01\x12\x0c\n\x08TEARDOWN\x10\x02\x12\n\n\x06ACCEPT\
    \x10\x03\x12\x0b\n\x07DECLINE\x10\x04\x12\x19\n\x15ACCEPT_INVERSE_OFFERS\
    \x10\r\x12\x1a\n\x16DECLINE_INVERSE_OFFERS\x10\x0e\x12\n\n\x06REVIVE\x10\
    \x05\x12\x08\n\x04KILL\x10\x06\x12\x0c\n\x08SHUTDOWN\x10\x07\x12\x0f\n\
    \x0bACKNOWLEDGE\x10\x08\x12\x20\n\x1cACKNOWLEDGE_OPERATION_STATUS\x10\
    \x0f\x12\r\n\tRECONCILE\x10\t\x12\x18\n\x14RECONCILE_OPERATIONS\x10\x10\
    \x12\x0b\n\x07MESSAGE\x10\n\x12\x0b\n\x07REQUEST\x10\x0b\x12\x0c\n\x08SU\
    PPRESS\x10\x0cB'\n\x1dorg.apache.mesos.v1.schedulerB\x06ProtosJ\xc4\xb1\
    \x01\n\x07\x12\x05\x10\0\xe9\x03\x01\n\x8c\x06\n\x01\x0c\x12\x03\x10\0\
    \x122\x81\x06\x20Licensed\x20to\x20the\x20Apache\x20Software\x20Foundati\
    on\x20(ASF)\x20under\x20one\n\x20or\x20more\x20contributor\x20license\
    \x20agreements.\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20\
    with\x20this\x20work\x20for\x20additional\x20information\n\x20regarding\
    \x20copyright\x20ownership.\x20\x20The\x20ASF\x20licenses\x20this\x20fil\
    e\n\x20to\x20you\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\n\x20\"License\");\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\n\x20with\x20the\x20License.\x20\x20You\
    \x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\
    \x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20\
    required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writi\
    ng,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20dis\
    tributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIE\
    S\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\t\n\x02\x03\0\x12\x03\x12\x07\x1d\n\x08\n\x01\x02\x12\
    \x03\x14\x08\x1a\n\x08\n\x01\x08\x12\x03\x16\06\n\x0b\n\x04\x08\xe7\x07\
    \0\x12\x03\x16\06\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x16\x07\x13\n\r\
    \n\x06\x08\xe7\x07\0\x02\0\x12\x03\x16\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x16\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x16\
    \x165\n\x08\n\x01\x08\x12\x03\x17\0'\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\
    \x17\0'\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x17\x07\x1b\n\r\n\x06\
    \x08\xe7\x07\x01\x02\0\x12\x03\x17\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03\x17\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\
    \x17\x1e&\n\xbd\x01\n\x02\x04\0\x12\x05!\0\xcc\x01\x01\x1a\xaf\x01*\n\
    \x20Scheduler\x20event\x20API.\n\n\x20An\x20event\x20is\x20described\x20\
    using\x20the\x20standard\x20protocol\x20buffer\x20\"union\"\n\x20trick,\
    \x20see:\n\x20https://developers.google.com/protocol-buffers/docs/techni\
    ques#union.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08\r\nU\n\x04\x04\0\x04\0\
    \x12\x04$\x02>\x03\x1aG\x20Possible\x20event\x20types,\x20followed\x20by\
    \x20message\x20definitions\x20if\n\x20applicable.\n\n\x0c\n\x05\x04\0\
    \x04\0\x01\x12\x03$\x07\x0b\n\xdc\x01\n\x06\x04\0\x04\0\x02\0\x12\x03)\
    \x04\x10\x1a\xcc\x01\x20This\x20must\x20be\x20the\x20first\x20enum\x20va\
    lue\x20in\x20this\x20list,\x20to\n\x20ensure\x20that\x20if\x20'type'\x20\
    is\x20not\x20set,\x20the\x20default\x20value\n\x20is\x20UNKNOWN.\x20This\
    \x20enables\x20enum\x20values\x20to\x20be\x20added\n\x20in\x20a\x20backw\
    ards-compatible\x20way.\x20See:\x20MESOS-4997.\n\n\x0e\n\x07\x04\0\x04\0\
    \x02\0\x01\x12\x03)\x04\x0b\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03)\
    \x0e\x0f\n(\n\x06\x04\0\x04\0\x02\x01\x12\x03+\x04\x13\"\x19\x20See\x20'\
    Subscribed'\x20below.\n\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03+\x04\
    \x0e\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03+\x11\x12\n$\n\x06\x04\0\
    \x04\0\x02\x02\x12\x03,\x04\x0f\"\x15\x20See\x20'Offers'\x20below.\n\n\
    \x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03,\x04\n\n\x0e\n\x07\x04\0\x04\
    \0\x02\x02\x02\x12\x03,\r\x0e\n+\n\x06\x04\0\x04\0\x02\x03\x12\x03-\x04\
    \x17\"\x1c\x20See\x20'InverseOffers'\x20below.\n\n\x0e\n\x07\x04\0\x04\0\
    \x02\x03\x01\x12\x03-\x04\x12\n\x0e\n\x07\x04\0\x04\0\x02\x03\x02\x12\
    \x03-\x15\x16\n%\n\x06\x04\0\x04\0\x02\x04\x12\x03.\x04\x10\"\x16\x20See\
    \x20'Rescind'\x20below.\n\n\x0e\n\x07\x04\0\x04\0\x02\x04\x01\x12\x03.\
    \x04\x0b\n\x0e\n\x07\x04\0\x04\0\x02\x04\x02\x12\x03.\x0e\x0f\n1\n\x06\
    \x04\0\x04\0\x02\x05\x12\x03/\x04\x1f\"\"\x20See\x20'RescindInverseOffer\
    '\x20below.\n\n\x0e\n\x07\x04\0\x04\0\x02\x05\x01\x12\x03/\x04\x19\n\x0e\
    \n\x07\x04\0\x04\0\x02\x05\x02\x12\x03/\x1c\x1e\n$\n\x06\x04\0\x04\0\x02\
    \x06\x12\x030\x04\x0f\"\x15\x20See\x20'Update'\x20below.\n\n\x0e\n\x07\
    \x04\0\x04\0\x02\x06\x01\x12\x030\x04\n\n\x0e\n\x07\x04\0\x04\0\x02\x06\
    \x02\x12\x030\r\x0e\n3\n\x06\x04\0\x04\0\x02\x07\x12\x031\x04!\"$\x20See\
    \x20'UpdateOperationStatus'\x20below.\n\n\x0e\n\x07\x04\0\x04\0\x02\x07\
    \x01\x12\x031\x04\x1b\n\x0e\n\x07\x04\0\x04\0\x02\x07\x02\x12\x031\x1e\
    \x20\n%\n\x06\x04\0\x04\0\x02\x08\x12\x032\x04\x10\"\x16\x20See\x20'Mess\
    age'\x20below.\n\n\x0e\n\x07\x04\0\x04\0\x02\x08\x01\x12\x032\x04\x0b\n\
    \x0e\n\x07\x04\0\x04\0\x02\x08\x02\x12\x032\x0e\x0f\n%\n\x06\x04\0\x04\0\
    \x02\t\x12\x033\x04\x10\"\x16\x20See\x20'Failure'\x20below.\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\t\x01\x12\x033\x04\x0b\n\x0e\n\x07\x04\0\x04\0\x02\
    \t\x02\x12\x033\x0e\x0f\n#\n\x06\x04\0\x04\0\x02\n\x12\x034\x04\x0e\"\
    \x14\x20See\x20'Error'\x20below.\n\n\x0e\n\x07\x04\0\x04\0\x02\n\x01\x12\
    \x034\x04\t\n\x0e\n\x07\x04\0\x04\0\x02\n\x02\x12\x034\x0c\r\n\xa0\x03\n\
    \x06\x04\0\x04\0\x02\x0b\x12\x03=\x04\x12\x1a\x90\x03\x20Periodic\x20mes\
    sage\x20sent\x20by\x20the\x20Mesos\x20master\x20according\x20to\n\x20'Su\
    bscribed.heartbeat_interval_seconds'.\x20If\x20the\x20scheduler\x20does\
    \n\x20not\x20receive\x20any\x20events\x20(including\x20heartbeats)\x20fo\
    r\x20an\x20extended\n\x20period\x20of\x20time\x20(e.g.,\x205\x20x\x20hea\
    rtbeat_interval_seconds),\x20there\x20is\n\x20likely\x20a\x20network\x20\
    partition.\x20In\x20such\x20a\x20case\x20the\x20scheduler\x20should\n\
    \x20close\x20the\x20existing\x20subscription\x20connection\x20and\x20res\
    ubscribe\n\x20using\x20a\x20backoff\x20strategy.\n\n\x0e\n\x07\x04\0\x04\
    \0\x02\x0b\x01\x12\x03=\x04\r\n\x0e\n\x07\x04\0\x04\0\x02\x0b\x02\x12\
    \x03=\x10\x11\nC\n\x04\x04\0\x03\0\x12\x04A\x02J\x03\x1a5\x20First\x20ev\
    ent\x20received\x20when\x20the\x20scheduler\x20subscribes.\n\n\x0c\n\x05\
    \x04\0\x03\0\x01\x12\x03A\n\x14\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03B\x04\
    *\n\x0e\n\x07\x04\0\x03\0\x02\0\x04\x12\x03B\x04\x0c\n\x0e\n\x07\x04\0\
    \x03\0\x02\0\x06\x12\x03B\r\x18\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\
    \x03B\x19%\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03B()\n\x84\x01\n\x06\
    \x04\0\x03\0\x02\x01\x12\x03F\x043\x1au\x20This\x20value\x20will\x20be\
    \x20set\x20if\x20the\x20master\x20is\x20sending\x20heartbeats.\x20See\n\
    \x20the\x20comment\x20above\x20on\x20'HEARTBEAT'\x20for\x20more\x20detai\
    ls.\n\n\x0e\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03F\x04\x0c\n\x0e\n\x07\
    \x04\0\x03\0\x02\x01\x05\x12\x03F\r\x13\n\x0e\n\x07\x04\0\x03\0\x02\x01\
    \x01\x12\x03F\x14.\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03F12\n!\n\
    \x06\x04\0\x03\0\x02\x02\x12\x03I\x04(\x1a\x12\x20Since\x20Mesos\x201.1.\
    \n\n\x0e\n\x07\x04\0\x03\0\x02\x02\x04\x12\x03I\x04\x0c\n\x0e\n\x07\x04\
    \0\x03\0\x02\x02\x06\x12\x03I\r\x17\n\x0e\n\x07\x04\0\x03\0\x02\x02\x01\
    \x12\x03I\x18#\n\x0e\n\x07\x04\0\x03\0\x02\x02\x03\x12\x03I&'\n\x84\x02\
    \n\x04\x04\0\x03\x01\x12\x04P\x02R\x03\x1a\xf5\x01\x20Received\x20whenev\
    er\x20there\x20are\x20new\x20resources\x20that\x20are\x20offered\x20to\
    \x20the\n\x20scheduler.\x20Each\x20offer\x20corresponds\x20to\x20a\x20se\
    t\x20of\x20resources\x20on\x20an\n\x20agent.\x20Until\x20the\x20schedule\
    r\x20accepts\x20or\x20declines\x20an\x20offer\x20the\n\x20resources\x20a\
    re\x20considered\x20allocated\x20to\x20the\x20scheduler.\n\n\x0c\n\x05\
    \x04\0\x03\x01\x01\x12\x03P\n\x10\n\r\n\x06\x04\0\x03\x01\x02\0\x12\x03Q\
    \x04\x1e\n\x0e\n\x07\x04\0\x03\x01\x02\0\x04\x12\x03Q\x04\x0c\n\x0e\n\
    \x07\x04\0\x03\x01\x02\0\x06\x12\x03Q\r\x12\n\x0e\n\x07\x04\0\x03\x01\
    \x02\0\x01\x12\x03Q\x13\x19\n\x0e\n\x07\x04\0\x03\x01\x02\0\x03\x12\x03Q\
    \x1c\x1d\n\x9c\x03\n\x04\x04\0\x03\x02\x12\x04[\x02]\x03\x1a\x8d\x03\x20\
    Received\x20whenever\x20there\x20are\x20resources\x20requested\x20back\
    \x20from\x20the\n\x20scheduler.\x20Each\x20inverse\x20offer\x20specifies\
    \x20the\x20agent,\x20and\n\x20optionally\x20specific\x20resources.\x20Ac\
    cepting\x20or\x20Declining\x20an\x20inverse\n\x20offer\x20informs\x20the\
    \x20allocator\x20of\x20the\x20scheduler's\x20ability\x20to\x20release\n\
    \x20the\x20specified\x20resources\x20without\x20violating\x20an\x20SLA.\
    \x20If\x20no\x20resources\n\x20are\x20specified\x20then\x20all\x20resour\
    ces\x20on\x20the\x20agent\x20are\x20requested\x20to\x20be\n\x20released.\
    \n\n\x0c\n\x05\x04\0\x03\x02\x01\x12\x03[\n\x17\n\r\n\x06\x04\0\x03\x02\
    \x02\0\x12\x03\\\x04-\n\x0e\n\x07\x04\0\x03\x02\x02\0\x04\x12\x03\\\x04\
    \x0c\n\x0e\n\x07\x04\0\x03\x02\x02\0\x06\x12\x03\\\r\x19\n\x0e\n\x07\x04\
    \0\x03\x02\x02\0\x01\x12\x03\\\x1a(\n\x0e\n\x07\x04\0\x03\x02\x02\0\x03\
    \x12\x03\\+,\n\x8a\x02\n\x04\x04\0\x03\x03\x12\x04c\x02e\x03\x1a\xfb\x01\
    \x20Received\x20when\x20a\x20particular\x20offer\x20is\x20no\x20longer\
    \x20valid\x20(e.g.,\x20the\n\x20agent\x20corresponding\x20to\x20the\x20o\
    ffer\x20has\x20been\x20removed)\x20and\x20hence\n\x20needs\x20to\x20be\
    \x20rescinded.\x20Any\x20future\x20calls\x20('Accept'\x20/\x20'Decline')\
    \x20made\n\x20by\x20the\x20scheduler\x20regarding\x20this\x20offer\x20wi\
    ll\x20be\x20invalid.\n\n\x0c\n\x05\x04\0\x03\x03\x01\x12\x03c\n\x11\n\r\
    \n\x06\x04\0\x03\x03\x02\0\x12\x03d\x04\"\n\x0e\n\x07\x04\0\x03\x03\x02\
    \0\x04\x12\x03d\x04\x0c\n\x0e\n\x07\x04\0\x03\x03\x02\0\x06\x12\x03d\r\
    \x14\n\x0e\n\x07\x04\0\x03\x03\x02\0\x01\x12\x03d\x15\x1d\n\x0e\n\x07\
    \x04\0\x03\x03\x02\0\x03\x12\x03d\x20!\n\x9b\x02\n\x04\x04\0\x03\x04\x12\
    \x04l\x02n\x03\x1a\x8c\x02\x20Received\x20when\x20a\x20particular\x20inv\
    erse\x20offer\x20is\x20no\x20longer\x20valid\n\x20(e.g.,\x20the\x20agent\
    \x20corresponding\x20to\x20the\x20offer\x20has\x20been\x20removed)\n\x20\
    and\x20hence\x20needs\x20to\x20be\x20rescinded.\x20Any\x20future\x20call\
    s\x20('Accept'\x20/\n\x20'Decline')\x20made\x20by\x20the\x20scheduler\
    \x20regarding\x20this\x20inverse\x20offer\n\x20will\x20be\x20invalid.\n\
    \n\x0c\n\x05\x04\0\x03\x04\x01\x12\x03l\n\x1d\n\r\n\x06\x04\0\x03\x04\
    \x02\0\x12\x03m\x04*\n\x0e\n\x07\x04\0\x03\x04\x02\0\x04\x12\x03m\x04\
    \x0c\n\x0e\n\x07\x04\0\x03\x04\x02\0\x06\x12\x03m\r\x14\n\x0e\n\x07\x04\
    \0\x03\x04\x02\0\x01\x12\x03m\x15%\n\x0e\n\x07\x04\0\x03\x04\x02\0\x03\
    \x12\x03m()\n\xcf\x06\n\x04\x04\0\x03\x05\x12\x05~\x02\x80\x01\x03\x1a\
    \xbf\x06\x20Received\x20whenever\x20there\x20is\x20a\x20status\x20update\
    \x20that\x20is\x20generated\x20by\n\x20the\x20executor\x20or\x20agent\
    \x20or\x20master.\x20Status\x20updates\x20should\x20be\x20used\x20by\n\
    \x20executors\x20to\x20reliably\x20communicate\x20the\x20status\x20of\
    \x20the\x20tasks\x20that\n\x20they\x20manage.\x20It\x20is\x20crucial\x20\
    that\x20a\x20terminal\x20update\x20(see\x20TaskState\n\x20in\x20v1/mesos\
    .proto)\x20is\x20sent\x20by\x20the\x20executor\x20as\x20soon\x20as\x20th\
    e\x20task\n\x20terminates,\x20in\x20order\x20for\x20Mesos\x20to\x20relea\
    se\x20the\x20resources\x20allocated\n\x20to\x20the\x20task.\x20It\x20is\
    \x20also\x20the\x20responsibility\x20of\x20the\x20scheduler\x20to\n\x20e\
    xplicitly\x20acknowledge\x20the\x20receipt\x20of\x20a\x20status\x20updat\
    e.\x20See\n\x20'Acknowledge'\x20in\x20the\x20'Call'\x20section\x20below\
    \x20for\x20the\x20semantics.\n\n\x20A\x20task\x20status\x20update\x20may\
    \x20be\x20used\x20for\x20guaranteed\x20delivery\x20of\x20some\n\x20task-\
    related\x20information,\x20e.g.,\x20task's\x20health\x20update.\x20Such\
    \n\x20information\x20may\x20be\x20shadowed\x20by\x20subsequent\x20task\
    \x20status\x20updates,\x20that\n\x20do\x20not\x20preserve\x20fields\x20o\
    f\x20the\x20previously\x20sent\x20message.\n\n\x0c\n\x05\x04\0\x03\x05\
    \x01\x12\x03~\n\x10\n\r\n\x06\x04\0\x03\x05\x02\0\x12\x03\x7f\x04#\n\x0e\
    \n\x07\x04\0\x03\x05\x02\0\x04\x12\x03\x7f\x04\x0c\n\x0e\n\x07\x04\0\x03\
    \x05\x02\0\x06\x12\x03\x7f\r\x17\n\x0e\n\x07\x04\0\x03\x05\x02\0\x01\x12\
    \x03\x7f\x18\x1e\n\x0e\n\x07\x04\0\x03\x05\x02\0\x03\x12\x03\x7f!\"\n\
    \xa1\x03\n\x04\x04\0\x03\x06\x12\x06\x89\x01\x02\x8b\x01\x03\x1a\x90\x03\
    \x20Received\x20when\x20there\x20is\x20an\x20operation\x20status\x20upda\
    te\x20generated\x20by\x20the\n\x20master,\x20agent,\x20or\x20resource\
    \x20provider.\x20These\x20updates\x20are\x20only\x20sent\x20to\n\x20the\
    \x20framework\x20for\x20operations\x20which\x20had\x20the\x20operation\
    \x20ID\x20set\x20by\x20the\n\x20framework.\x20It\x20is\x20the\x20respons\
    ibility\x20of\x20the\x20scheduler\x20to\x20explicitly\n\x20acknowledge\
    \x20the\x20receipt\x20of\x20a\x20status\x20update.\n\x20See\x20'Acknowle\
    dgeOperationStatus'\x20in\x20the\x20'Call'\x20section\x20below\x20for\n\
    \x20the\x20semantics.\n\n\r\n\x05\x04\0\x03\x06\x01\x12\x04\x89\x01\n\
    \x1f\n\x0e\n\x06\x04\0\x03\x06\x02\0\x12\x04\x8a\x01\x04(\n\x0f\n\x07\
    \x04\0\x03\x06\x02\0\x04\x12\x04\x8a\x01\x04\x0c\n\x0f\n\x07\x04\0\x03\
    \x06\x02\0\x06\x12\x04\x8a\x01\r\x1c\n\x0f\n\x07\x04\0\x03\x06\x02\0\x01\
    \x12\x04\x8a\x01\x1d#\n\x0f\n\x07\x04\0\x03\x06\x02\0\x03\x12\x04\x8a\
    \x01&'\n\xb3\x02\n\x04\x04\0\x03\x07\x12\x06\x92\x01\x02\x96\x01\x03\x1a\
    \xa2\x02\x20Received\x20when\x20a\x20custom\x20message\x20generated\x20b\
    y\x20the\x20executor\x20is\n\x20forwarded\x20by\x20the\x20master.\x20Not\
    e\x20that\x20this\x20message\x20is\x20not\n\x20interpreted\x20by\x20Meso\
    s\x20and\x20is\x20only\x20forwarded\x20(without\x20reliability\n\x20guar\
    antees)\x20to\x20the\x20scheduler.\x20It\x20is\x20up\x20to\x20the\x20exe\
    cutor\x20to\x20retry\n\x20if\x20the\x20message\x20is\x20dropped\x20for\
    \x20any\x20reason.\n\n\r\n\x05\x04\0\x03\x07\x01\x12\x04\x92\x01\n\x11\n\
    \x0e\n\x06\x04\0\x03\x07\x02\0\x12\x04\x93\x01\x04\"\n\x0f\n\x07\x04\0\
    \x03\x07\x02\0\x04\x12\x04\x93\x01\x04\x0c\n\x0f\n\x07\x04\0\x03\x07\x02\
    \0\x06\x12\x04\x93\x01\r\x14\n\x0f\n\x07\x04\0\x03\x07\x02\0\x01\x12\x04\
    \x93\x01\x15\x1d\n\x0f\n\x07\x04\0\x03\x07\x02\0\x03\x12\x04\x93\x01\x20\
    !\n\x0e\n\x06\x04\0\x03\x07\x02\x01\x12\x04\x94\x01\x04(\n\x0f\n\x07\x04\
    \0\x03\x07\x02\x01\x04\x12\x04\x94\x01\x04\x0c\n\x0f\n\x07\x04\0\x03\x07\
    \x02\x01\x06\x12\x04\x94\x01\r\x17\n\x0f\n\x07\x04\0\x03\x07\x02\x01\x01\
    \x12\x04\x94\x01\x18#\n\x0f\n\x07\x04\0\x03\x07\x02\x01\x03\x12\x04\x94\
    \x01&'\n\x0e\n\x06\x04\0\x03\x07\x02\x02\x12\x04\x95\x01\x04\x1c\n\x0f\n\
    \x07\x04\0\x03\x07\x02\x02\x04\x12\x04\x95\x01\x04\x0c\n\x0f\n\x07\x04\0\
    \x03\x07\x02\x02\x05\x12\x04\x95\x01\r\x12\n\x0f\n\x07\x04\0\x03\x07\x02\
    \x02\x01\x12\x04\x95\x01\x13\x17\n\x0f\n\x07\x04\0\x03\x07\x02\x02\x03\
    \x12\x04\x95\x01\x1a\x1b\n\xe2\x04\n\x04\x04\0\x03\x08\x12\x06\xa2\x01\
    \x02\xb4\x01\x03\x1a\xd1\x04\x20Received\x20when\x20an\x20agent\x20is\
    \x20removed\x20from\x20the\x20cluster\x20(e.g.,\x20failed\n\x20health\
    \x20checks)\x20or\x20when\x20an\x20executor\x20is\x20terminated.\x20Note\
    \x20that,\x20this\n\x20event\x20coincides\x20with\x20receipt\x20of\x20te\
    rminal\x20UPDATE\x20events\x20for\x20any\n\x20active\x20tasks\x20belongi\
    ng\x20to\x20the\x20agent\x20or\x20executor\x20and\x20receipt\x20of\n\x20\
    'Rescind'\x20events\x20for\x20any\x20outstanding\x20offers\x20belonging\
    \x20to\x20the\n\x20agent.\x20Note\x20that\x20there\x20is\x20no\x20guaran\
    teed\x20order\x20between\x20the\n\x20'Failure',\x20'Update'\x20and\x20'R\
    escind'\x20events\x20when\x20an\x20agent\x20or\x20executor\n\x20is\x20re\
    moved.\n\x20TODO(vinod):\x20Consider\x20splitting\x20the\x20lost\x20agen\
    t\x20and\x20terminated\n\x20executor\x20into\x20separate\x20events\x20an\
    d\x20ensure\x20it's\x20reliably\x20generated.\n\n\r\n\x05\x04\0\x03\x08\
    \x01\x12\x04\xa2\x01\n\x11\n\x0e\n\x06\x04\0\x03\x08\x02\0\x12\x04\xa3\
    \x01\x04\"\n\x0f\n\x07\x04\0\x03\x08\x02\0\x04\x12\x04\xa3\x01\x04\x0c\n\
    \x0f\n\x07\x04\0\x03\x08\x02\0\x06\x12\x04\xa3\x01\r\x14\n\x0f\n\x07\x04\
    \0\x03\x08\x02\0\x01\x12\x04\xa3\x01\x15\x1d\n\x0f\n\x07\x04\0\x03\x08\
    \x02\0\x03\x12\x04\xa3\x01\x20!\n\xaf\x01\n\x06\x04\0\x03\x08\x02\x01\
    \x12\x04\xa8\x01\x04(\x1a\x9e\x01\x20If\x20this\x20was\x20just\x20a\x20f\
    ailure\x20of\x20an\x20executor\x20on\x20an\x20agent\x20then\n\x20'execut\
    or_id'\x20will\x20be\x20set\x20and\x20possibly\x20'status'\x20(if\x20we\
    \x20were\n\x20able\x20to\x20determine\x20the\x20exit\x20status).\n\n\x0f\
    \n\x07\x04\0\x03\x08\x02\x01\x04\x12\x04\xa8\x01\x04\x0c\n\x0f\n\x07\x04\
    \0\x03\x08\x02\x01\x06\x12\x04\xa8\x01\r\x17\n\x0f\n\x07\x04\0\x03\x08\
    \x02\x01\x01\x12\x04\xa8\x01\x18#\n\x0f\n\x07\x04\0\x03\x08\x02\x01\x03\
    \x12\x04\xa8\x01&'\n\xd4\x03\n\x06\x04\0\x03\x08\x02\x02\x12\x04\xb3\x01\
    \x04\x1e\x1a\xc3\x03\x20On\x20Posix,\x20`status`\x20corresponds\x20to\
    \x20termination\x20information\x20in\x20the\n\x20`stat_loc`\x20area\x20r\
    eturned\x20from\x20a\x20`waitpid`\x20call.\x20On\x20Windows,\x20`status`\
    \n\x20is\x20obtained\x20via\x20calling\x20the\x20`GetExitCodeProcess()`\
    \x20function.\x20For\n\x20messages\x20coming\x20from\x20Posix\x20agents,\
    \x20schedulers\x20need\x20to\x20apply\n\x20`WEXITSTATUS`\x20family\x20ma\
    cros\x20or\x20equivalent\x20transformations\x20to\x20obtain\n\x20exit\
    \x20codes.\n\n\x20TODO(alexr):\x20Consider\x20unifying\x20Windows\x20and\
    \x20Posix\x20behavior\x20by\x20returning\n\x20exit\x20code\x20here,\x20s\
    ee\x20MESOS-7241.\n\n\x0f\n\x07\x04\0\x03\x08\x02\x02\x04\x12\x04\xb3\
    \x01\x04\x0c\n\x0f\n\x07\x04\0\x03\x08\x02\x02\x05\x12\x04\xb3\x01\r\x12\
    \n\x0f\n\x07\x04\0\x03\x08\x02\x02\x01\x12\x04\xb3\x01\x13\x19\n\x0f\n\
    \x07\x04\0\x03\x08\x02\x02\x03\x12\x04\xb3\x01\x1c\x1d\n\xd0\x01\n\x04\
    \x04\0\x03\t\x12\x06\xb9\x01\x02\xbb\x01\x03\x1a\xbf\x01\x20Received\x20\
    when\x20there\x20is\x20an\x20unrecoverable\x20error\x20in\x20the\x20sche\
    duler\x20(e.g.,\n\x20scheduler\x20failed\x20over,\x20rate\x20limiting,\
    \x20authorization\x20errors\x20etc.).\x20The\n\x20scheduler\x20should\
    \x20abort\x20on\x20receiving\x20this\x20event.\n\n\r\n\x05\x04\0\x03\t\
    \x01\x12\x04\xb9\x01\n\x0f\n\x0e\n\x06\x04\0\x03\t\x02\0\x12\x04\xba\x01\
    \x04\x20\n\x0f\n\x07\x04\0\x03\t\x02\0\x04\x12\x04\xba\x01\x04\x0c\n\x0f\
    \n\x07\x04\0\x03\t\x02\0\x05\x12\x04\xba\x01\r\x13\n\x0f\n\x07\x04\0\x03\
    \t\x02\0\x01\x12\x04\xba\x01\x14\x1b\n\x0f\n\x07\x04\0\x03\t\x02\0\x03\
    \x12\x04\xba\x01\x1e\x1f\n\xbb\x01\n\x04\x04\0\x02\0\x12\x04\xc0\x01\x02\
    \x19\x1a\xac\x01\x20Type\x20of\x20the\x20event,\x20indicates\x20which\
    \x20optional\x20field\x20below\x20should\x20be\n\x20present\x20if\x20tha\
    t\x20type\x20has\x20a\x20nested\x20message\x20definition.\n\x20Enum\x20f\
    ields\x20should\x20be\x20optional,\x20see:\x20MESOS-4997.\n\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\xc0\x01\x02\n\n\r\n\x05\x04\0\x02\0\x06\x12\x04\
    \xc0\x01\x0b\x0f\n\r\n\x05\x04\0\x02\0\x01\x12\x04\xc0\x01\x10\x14\n\r\n\
    \x05\x04\0\x02\0\x03\x12\x04\xc0\x01\x17\x18\n\x0c\n\x04\x04\0\x02\x01\
    \x12\x04\xc2\x01\x02%\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\xc2\x01\x02\n\
    \n\r\n\x05\x04\0\x02\x01\x06\x12\x04\xc2\x01\x0b\x15\n\r\n\x05\x04\0\x02\
    \x01\x01\x12\x04\xc2\x01\x16\x20\n\r\n\x05\x04\0\x02\x01\x03\x12\x04\xc2\
    \x01#$\n\x0c\n\x04\x04\0\x02\x02\x12\x04\xc3\x01\x02\x1d\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\xc3\x01\x02\n\n\r\n\x05\x04\0\x02\x02\x06\x12\x04\
    \xc3\x01\x0b\x11\n\r\n\x05\x04\0\x02\x02\x01\x12\x04\xc3\x01\x12\x18\n\r\
    \n\x05\x04\0\x02\x02\x03\x12\x04\xc3\x01\x1b\x1c\n\x0c\n\x04\x04\0\x02\
    \x03\x12\x04\xc4\x01\x02,\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\xc4\x01\
    \x02\n\n\r\n\x05\x04\0\x02\x03\x06\x12\x04\xc4\x01\x0b\x18\n\r\n\x05\x04\
    \0\x02\x03\x01\x12\x04\xc4\x01\x19'\n\r\n\x05\x04\0\x02\x03\x03\x12\x04\
    \xc4\x01*+\n\x0c\n\x04\x04\0\x02\x04\x12\x04\xc5\x01\x02\x1f\n\r\n\x05\
    \x04\0\x02\x04\x04\x12\x04\xc5\x01\x02\n\n\r\n\x05\x04\0\x02\x04\x06\x12\
    \x04\xc5\x01\x0b\x12\n\r\n\x05\x04\0\x02\x04\x01\x12\x04\xc5\x01\x13\x1a\
    \n\r\n\x05\x04\0\x02\x04\x03\x12\x04\xc5\x01\x1d\x1e\n\x0c\n\x04\x04\0\
    \x02\x05\x12\x04\xc6\x01\x02:\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\xc6\
    \x01\x02\n\n\r\n\x05\x04\0\x02\x05\x06\x12\x04\xc6\x01\x0b\x1e\n\r\n\x05\
    \x04\0\x02\x05\x01\x12\x04\xc6\x01\x1f4\n\r\n\x05\x04\0\x02\x05\x03\x12\
    \x04\xc6\x0179\n\x0c\n\x04\x04\0\x02\x06\x12\x04\xc7\x01\x02\x1d\n\r\n\
    \x05\x04\0\x02\x06\x04\x12\x04\xc7\x01\x02\n\n\r\n\x05\x04\0\x02\x06\x06\
    \x12\x04\xc7\x01\x0b\x11\n\r\n\x05\x04\0\x02\x06\x01\x12\x04\xc7\x01\x12\
    \x18\n\r\n\x05\x04\0\x02\x06\x03\x12\x04\xc7\x01\x1b\x1c\n\x0c\n\x04\x04\
    \0\x02\x07\x12\x04\xc8\x01\x02>\n\r\n\x05\x04\0\x02\x07\x04\x12\x04\xc8\
    \x01\x02\n\n\r\n\x05\x04\0\x02\x07\x06\x12\x04\xc8\x01\x0b\x20\n\r\n\x05\
    \x04\0\x02\x07\x01\x12\x04\xc8\x01!8\n\r\n\x05\x04\0\x02\x07\x03\x12\x04\
    \xc8\x01;=\n\x0c\n\x04\x04\0\x02\x08\x12\x04\xc9\x01\x02\x1f\n\r\n\x05\
    \x04\0\x02\x08\x04\x12\x04\xc9\x01\x02\n\n\r\n\x05\x04\0\x02\x08\x06\x12\
    \x04\xc9\x01\x0b\x12\n\r\n\x05\x04\0\x02\x08\x01\x12\x04\xc9\x01\x13\x1a\
    \n\r\n\x05\x04\0\x02\x08\x03\x12\x04\xc9\x01\x1d\x1e\n\x0c\n\x04\x04\0\
    \x02\t\x12\x04\xca\x01\x02\x1f\n\r\n\x05\x04\0\x02\t\x04\x12\x04\xca\x01\
    \x02\n\n\r\n\x05\x04\0\x02\t\x06\x12\x04\xca\x01\x0b\x12\n\r\n\x05\x04\0\
    \x02\t\x01\x12\x04\xca\x01\x13\x1a\n\r\n\x05\x04\0\x02\t\x03\x12\x04\xca\
    \x01\x1d\x1e\n\x0c\n\x04\x04\0\x02\n\x12\x04\xcb\x01\x02\x1b\n\r\n\x05\
    \x04\0\x02\n\x04\x12\x04\xcb\x01\x02\n\n\r\n\x05\x04\0\x02\n\x06\x12\x04\
    \xcb\x01\x0b\x10\n\r\n\x05\x04\0\x02\n\x01\x12\x04\xcb\x01\x11\x16\n\r\n\
    \x05\x04\0\x02\n\x03\x12\x04\xcb\x01\x19\x1a\nL\n\x02\x04\x01\x12\x06\
    \xd2\x01\0\xdf\x01\x01\x1a>*\n\x20Synchronous\x20responses\x20for\x20cal\
    ls\x20made\x20to\x20the\x20scheduler\x20API.\n\n\x0b\n\x03\x04\x01\x01\
    \x12\x04\xd2\x01\x08\x10\nY\n\x04\x04\x01\x04\0\x12\x06\xd4\x01\x02\xd8\
    \x01\x03\x1aI\x20Each\x20of\x20the\x20responses\x20of\x20type\x20`FOO`\
    \x20corresponds\x20to\x20`Foo`\x20message\x20below.\n\n\r\n\x05\x04\x01\
    \x04\0\x01\x12\x04\xd4\x01\x07\x0b\n\x0e\n\x06\x04\x01\x04\0\x02\0\x12\
    \x04\xd5\x01\x04\x10\n\x0f\n\x07\x04\x01\x04\0\x02\0\x01\x12\x04\xd5\x01\
    \x04\x0b\n\x0f\n\x07\x04\x01\x04\0\x02\0\x02\x12\x04\xd5\x01\x0e\x0f\n2\
    \n\x06\x04\x01\x04\0\x02\x01\x12\x04\xd7\x01\x04\x1d\"\"\x20See\x20'Reco\
    ncileOperations'\x20below.\n\n\x0f\n\x07\x04\x01\x04\0\x02\x01\x01\x12\
    \x04\xd7\x01\x04\x18\n\x0f\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x04\xd7\
    \x01\x1b\x1c\n\x0e\n\x04\x04\x01\x03\0\x12\x06\xda\x01\x02\xdc\x01\x03\n\
    \r\n\x05\x04\x01\x03\0\x01\x12\x04\xda\x01\n\x1d\n\x0e\n\x06\x04\x01\x03\
    \0\x02\0\x12\x04\xdb\x01\x044\n\x0f\n\x07\x04\x01\x03\0\x02\0\x04\x12\
    \x04\xdb\x01\x04\x0c\n\x0f\n\x07\x04\x01\x03\0\x02\0\x06\x12\x04\xdb\x01\
    \r\x1c\n\x0f\n\x07\x04\x01\x03\0\x02\0\x01\x12\x04\xdb\x01\x1d/\n\x0f\n\
    \x07\x04\x01\x03\0\x02\0\x03\x12\x04\xdb\x0123\n\x0c\n\x04\x04\x01\x02\0\
    \x12\x04\xde\x01\x028\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\xde\x01\x02\n\
    \n\r\n\x05\x04\x01\x02\0\x06\x12\x04\xde\x01\x0b\x1e\n\r\n\x05\x04\x01\
    \x02\0\x01\x12\x04\xde\x01\x1f3\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\xde\
    \x0167\n\x86\x01\n\x02\x04\x02\x12\x06\xe8\x01\0\xe9\x03\x01\x1ax*\n\x20\
    Scheduler\x20call\x20API.\n\n\x20Like\x20Event,\x20a\x20Call\x20is\x20de\
    scribed\x20using\x20the\x20standard\x20protocol\x20buffer\n\x20\"union\"\
    \x20trick\x20(see\x20above).\n\n\x0b\n\x03\x04\x02\x01\x12\x04\xe8\x01\
    \x08\x0c\nV\n\x04\x04\x02\x04\0\x12\x06\xeb\x01\x02\x87\x02\x03\x1aF\x20\
    Possible\x20call\x20types,\x20followed\x20by\x20message\x20definitions\
    \x20if\n\x20applicable.\n\n\r\n\x05\x04\x02\x04\0\x01\x12\x04\xeb\x01\
    \x07\x0b\nZ\n\x06\x04\x02\x04\0\x02\0\x12\x04\xed\x01\x04\x10\x1aJ\x20Se\
    e\x20comments\x20above\x20on\x20`Event::Type`\x20for\x20more\x20details\
    \x20on\x20this\x20enum\x20value.\n\n\x0f\n\x07\x04\x02\x04\0\x02\0\x01\
    \x12\x04\xed\x01\x04\x0b\n\x0f\n\x07\x04\x02\x04\0\x02\0\x02\x12\x04\xed\
    \x01\x0e\x0f\n(\n\x06\x04\x02\x04\0\x02\x01\x12\x04\xef\x01\x04\x12\"\
    \x18\x20See\x20'Subscribe'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x01\
    \x01\x12\x04\xef\x01\x04\r\n\x0f\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x04\
    \xef\x01\x10\x11\nG\n\x06\x04\x02\x04\0\x02\x02\x12\x04\xf0\x01\x04\x11\
    \"7\x20Shuts\x20down\x20all\x20tasks/executors\x20and\x20removes\x20fram\
    ework.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x02\x01\x12\x04\xf0\x01\x04\x0c\n\
    \x0f\n\x07\x04\x02\x04\0\x02\x02\x02\x12\x04\xf0\x01\x0f\x10\n%\n\x06\
    \x04\x02\x04\0\x02\x03\x12\x04\xf1\x01\x04\x0f\"\x15\x20See\x20'Accept'\
    \x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x03\x01\x12\x04\xf1\x01\x04\n\
    \n\x0f\n\x07\x04\x02\x04\0\x02\x03\x02\x12\x04\xf1\x01\r\x0e\n&\n\x06\
    \x04\x02\x04\0\x02\x04\x12\x04\xf2\x01\x04\x10\"\x16\x20See\x20'Decline'\
    \x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x04\x01\x12\x04\xf2\x01\x04\
    \x0b\n\x0f\n\x07\x04\x02\x04\0\x02\x04\x02\x12\x04\xf2\x01\x0e\x0f\n2\n\
    \x06\x04\x02\x04\0\x02\x05\x12\x04\xf3\x01\x04\x1f\"\"\x20See\x20'Accept\
    InverseOffers'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x05\x01\x12\x04\
    \xf3\x01\x04\x19\n\x0f\n\x07\x04\x02\x04\0\x02\x05\x02\x12\x04\xf3\x01\
    \x1c\x1e\n3\n\x06\x04\x02\x04\0\x02\x06\x12\x04\xf4\x01\x04\x20\"#\x20Se\
    e\x20'DeclineInverseOffers'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\
    \x06\x01\x12\x04\xf4\x01\x04\x1a\n\x0f\n\x07\x04\x02\x04\0\x02\x06\x02\
    \x12\x04\xf4\x01\x1d\x1f\nI\n\x06\x04\x02\x04\0\x02\x07\x12\x04\xf5\x01\
    \x04\x0f\"9\x20Removes\x20any\x20previous\x20filters\x20set\x20via\x20AC\
    CEPT\x20or\x20DECLINE.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x07\x01\x12\x04\
    \xf5\x01\x04\n\n\x0f\n\x07\x04\x02\x04\0\x02\x07\x02\x12\x04\xf5\x01\r\
    \x0e\n#\n\x06\x04\x02\x04\0\x02\x08\x12\x04\xf6\x01\x04\r\"\x13\x20See\
    \x20'Kill'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x08\x01\x12\x04\xf6\
    \x01\x04\x08\n\x0f\n\x07\x04\x02\x04\0\x02\x08\x02\x12\x04\xf6\x01\x0b\
    \x0c\n'\n\x06\x04\x02\x04\0\x02\t\x12\x04\xf7\x01\x04\x11\"\x17\x20See\
    \x20'Shutdown'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\t\x01\x12\x04\
    \xf7\x01\x04\x0c\n\x0f\n\x07\x04\x02\x04\0\x02\t\x02\x12\x04\xf7\x01\x0f\
    \x10\n*\n\x06\x04\x02\x04\0\x02\n\x12\x04\xf8\x01\x04\x14\"\x1a\x20See\
    \x20'Acknowledge'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\n\x01\x12\
    \x04\xf8\x01\x04\x0f\n\x0f\n\x07\x04\x02\x04\0\x02\n\x02\x12\x04\xf8\x01\
    \x12\x13\n$\n\x06\x04\x02\x04\0\x02\x0b\x12\x04\xf9\x01\x04&\"\x14\x20Se\
    e\x20message\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x0b\x01\x12\x04\
    \xf9\x01\x04\x20\n\x0f\n\x07\x04\x02\x04\0\x02\x0b\x02\x12\x04\xf9\x01#%\
    \n(\n\x06\x04\x02\x04\0\x02\x0c\x12\x04\xfa\x01\x04\x12\"\x18\x20See\x20\
    'Reconcile'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x0c\x01\x12\x04\
    \xfa\x01\x04\r\n\x0f\n\x07\x04\x02\x04\0\x02\x0c\x02\x12\x04\xfa\x01\x10\
    \x11\n2\n\x06\x04\x02\x04\0\x02\r\x12\x04\xfb\x01\x04\x1e\"\"\x20See\x20\
    'ReconcileOperations'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\r\x01\
    \x12\x04\xfb\x01\x04\x18\n\x0f\n\x07\x04\x02\x04\0\x02\r\x02\x12\x04\xfb\
    \x01\x1b\x1d\n&\n\x06\x04\x02\x04\0\x02\x0e\x12\x04\xfc\x01\x04\x11\"\
    \x16\x20See\x20'Message'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x0e\
    \x01\x12\x04\xfc\x01\x04\x0b\n\x0f\n\x07\x04\x02\x04\0\x02\x0e\x02\x12\
    \x04\xfc\x01\x0e\x10\n&\n\x06\x04\x02\x04\0\x02\x0f\x12\x04\xfd\x01\x04\
    \x11\"\x16\x20See\x20'Request'\x20below.\n\n\x0f\n\x07\x04\x02\x04\0\x02\
    \x0f\x01\x12\x04\xfd\x01\x04\x0b\n\x0f\n\x07\x04\x02\x04\0\x02\x0f\x02\
    \x12\x04\xfd\x01\x0e\x10\nH\n\x06\x04\x02\x04\0\x02\x10\x12\x04\xfe\x01\
    \x04\x12\"8\x20Inform\x20master\x20to\x20stop\x20sending\x20offers\x20to\
    \x20the\x20framework.\n\n\x0f\n\x07\x04\x02\x04\0\x02\x10\x01\x12\x04\
    \xfe\x01\x04\x0c\n\x0f\n\x07\x04\x02\x04\0\x02\x10\x02\x12\x04\xfe\x01\
    \x0f\x11\n\xa2\x01\n\x04\x04\x02\x03\0\x12\x06\x8c\x02\x02\x95\x02\x03\
    \x1a\x91\x01\x20Subscribes\x20the\x20scheduler\x20with\x20the\x20master\
    \x20to\x20receive\x20events.\x20A\n\x20scheduler\x20must\x20send\x20othe\
    r\x20calls\x20only\x20after\x20it\x20has\x20received\x20the\n\x20SUBCRIB\
    ED\x20event.\n\n\r\n\x05\x04\x02\x03\0\x01\x12\x04\x8c\x02\n\x13\ne\n\
    \x06\x04\x02\x03\0\x02\0\x12\x04\x8f\x02\x04.\x1aU\x20See\x20the\x20comm\
    ents\x20below\x20on\x20'framework_id'\x20on\x20the\x20semantics\x20for\n\
    \x20'framework_info.id'.\n\n\x0f\n\x07\x04\x02\x03\0\x02\0\x04\x12\x04\
    \x8f\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\0\x02\0\x06\x12\x04\x8f\x02\r\
    \x1a\n\x0f\n\x07\x04\x02\x03\0\x02\0\x01\x12\x04\x8f\x02\x1b)\n\x0f\n\
    \x07\x04\x02\x03\0\x02\0\x03\x12\x04\x8f\x02,-\n\xca\x01\n\x06\x04\x02\
    \x03\0\x02\x01\x12\x04\x94\x02\x04)\x1a\xb9\x01\x20List\x20of\x20suppres\
    sed\x20roles\x20for\x20which\x20the\x20framework\x20does\x20not\x20wish\
    \x20to\x20be\n\x20offered\x20resources.\x20The\x20framework\x20can\x20de\
    cide\x20to\x20suppress\x20all\x20or\x20a\x20subset\n\x20of\x20roles\x20t\
    he\x20framework\x20(re)registers\x20as.\n\n\x0f\n\x07\x04\x02\x03\0\x02\
    \x01\x04\x12\x04\x94\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x05\
    \x12\x04\x94\x02\r\x13\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x04\x94\
    \x02\x14$\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x04\x94\x02'(\n\xdd\
    \x05\n\x04\x04\x02\x03\x01\x12\x06\xac\x02\x02\xb0\x02\x03\x1a\xcc\x05\
    \x20Accepts\x20an\x20offer,\x20performing\x20the\x20specified\x20operati\
    ons\n\x20in\x20a\x20sequential\x20manner.\n\n\x20E.g.\x20Launch\x20a\x20\
    task\x20with\x20a\x20newly\x20reserved\x20persistent\x20volume:\n\n\x20\
    \x20\x20Accept\x20{\n\x20\x20\x20\x20\x20offer_ids:\x20[\x20...\x20]\n\
    \x20\x20\x20\x20\x20operations:\x20[\n\x20\x20\x20\x20\x20\x20\x20{\x20t\
    ype:\x20RESERVE,\n\x20\x20\x20\x20\x20\x20\x20\x20\x20reserve:\x20{\x20r\
    esources:\x20[\x20disk(role):2\x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\
    \x20{\x20type:\x20CREATE,\n\x20\x20\x20\x20\x20\x20\x20\x20\x20create:\
    \x20{\x20volumes:\x20[\x20disk(role):1+persistence\x20]\x20}\x20}\n\x20\
    \x20\x20\x20\x20\x20\x20{\x20type:\x20LAUNCH,\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20launch:\x20{\x20task_infos\x20...\x20disk(role):1;disk(role)\
    :1+persistence\x20}\x20}\n\x20\x20\x20\x20\x20]\n\x20\x20\x20}\n\n\x20No\
    te\x20that\x20any\x20of\x20the\x20offer\xe2\x80\x99s\x20resources\x20not\
    \x20used\x20in\x20the\x20'Accept'\n\x20call\x20(e.g.,\x20to\x20launch\
    \x20a\x20task)\x20are\x20considered\x20unused\x20and\x20might\x20be\n\
    \x20reoffered\x20to\x20other\x20frameworks.\x20In\x20other\x20words,\x20\
    the\x20same\x20OfferID\n\x20cannot\x20be\x20used\x20in\x20more\x20than\
    \x20one\x20'Accept'\x20call.\n\n\r\n\x05\x04\x02\x03\x01\x01\x12\x04\xac\
    \x02\n\x10\n\x0e\n\x06\x04\x02\x03\x01\x02\0\x12\x04\xad\x02\x04#\n\x0f\
    \n\x07\x04\x02\x03\x01\x02\0\x04\x12\x04\xad\x02\x04\x0c\n\x0f\n\x07\x04\
    \x02\x03\x01\x02\0\x06\x12\x04\xad\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x01\
    \x02\0\x01\x12\x04\xad\x02\x15\x1e\n\x0f\n\x07\x04\x02\x03\x01\x02\0\x03\
    \x12\x04\xad\x02!\"\n\x0e\n\x06\x04\x02\x03\x01\x02\x01\x12\x04\xae\x02\
    \x04,\n\x0f\n\x07\x04\x02\x03\x01\x02\x01\x04\x12\x04\xae\x02\x04\x0c\n\
    \x0f\n\x07\x04\x02\x03\x01\x02\x01\x06\x12\x04\xae\x02\r\x1c\n\x0f\n\x07\
    \x04\x02\x03\x01\x02\x01\x01\x12\x04\xae\x02\x1d'\n\x0f\n\x07\x04\x02\
    \x03\x01\x02\x01\x03\x12\x04\xae\x02*+\n\x0e\n\x06\x04\x02\x03\x01\x02\
    \x02\x12\x04\xaf\x02\x04!\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x04\x12\
    \x04\xaf\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x06\x12\x04\xaf\
    \x02\r\x14\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x01\x12\x04\xaf\x02\x15\
    \x1c\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x03\x12\x04\xaf\x02\x1f\x20\n\
    \xf0\x01\n\x04\x04\x02\x03\x02\x12\x06\xb6\x02\x02\xb9\x02\x03\x1a\xdf\
    \x01\x20Declines\x20an\x20offer,\x20signaling\x20the\x20master\x20to\x20\
    potentially\x20reoffer\n\x20the\x20resources\x20to\x20a\x20different\x20\
    framework.\x20Note\x20that\x20this\x20is\x20same\n\x20as\x20sending\x20a\
    n\x20Accept\x20call\x20with\x20no\x20operations.\x20See\x20comments\x20o\
    n\n\x20top\x20of\x20'Accept'\x20for\x20semantics.\n\n\r\n\x05\x04\x02\
    \x03\x02\x01\x12\x04\xb6\x02\n\x11\n\x0e\n\x06\x04\x02\x03\x02\x02\0\x12\
    \x04\xb7\x02\x04#\n\x0f\n\x07\x04\x02\x03\x02\x02\0\x04\x12\x04\xb7\x02\
    \x04\x0c\n\x0f\n\x07\x04\x02\x03\x02\x02\0\x06\x12\x04\xb7\x02\r\x14\n\
    \x0f\n\x07\x04\x02\x03\x02\x02\0\x01\x12\x04\xb7\x02\x15\x1e\n\x0f\n\x07\
    \x04\x02\x03\x02\x02\0\x03\x12\x04\xb7\x02!\"\n\x0e\n\x06\x04\x02\x03\
    \x02\x02\x01\x12\x04\xb8\x02\x04!\n\x0f\n\x07\x04\x02\x03\x02\x02\x01\
    \x04\x12\x04\xb8\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x02\x02\x01\x06\x12\
    \x04\xb8\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x02\x02\x01\x01\x12\x04\xb8\
    \x02\x15\x1c\n\x0f\n\x07\x04\x02\x03\x02\x02\x01\x03\x12\x04\xb8\x02\x1f\
    \x20\n\xaf\x01\n\x04\x04\x02\x03\x03\x12\x06\xbe\x02\x02\xc1\x02\x03\x1a\
    \x9e\x01\x20Accepts\x20an\x20inverse\x20offer.\x20Inverse\x20offers\x20s\
    hould\x20only\x20be\x20accepted\n\x20if\x20the\x20resources\x20in\x20the\
    \x20offer\x20can\x20be\x20safely\x20evacuated\x20before\x20the\n\x20prov\
    ided\x20unavailability.\n\n\r\n\x05\x04\x02\x03\x03\x01\x12\x04\xbe\x02\
    \n\x1d\n\x0e\n\x06\x04\x02\x03\x03\x02\0\x12\x04\xbf\x02\x04+\n\x0f\n\
    \x07\x04\x02\x03\x03\x02\0\x04\x12\x04\xbf\x02\x04\x0c\n\x0f\n\x07\x04\
    \x02\x03\x03\x02\0\x06\x12\x04\xbf\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x03\
    \x02\0\x01\x12\x04\xbf\x02\x15&\n\x0f\n\x07\x04\x02\x03\x03\x02\0\x03\
    \x12\x04\xbf\x02)*\n\x0e\n\x06\x04\x02\x03\x03\x02\x01\x12\x04\xc0\x02\
    \x04!\n\x0f\n\x07\x04\x02\x03\x03\x02\x01\x04\x12\x04\xc0\x02\x04\x0c\n\
    \x0f\n\x07\x04\x02\x03\x03\x02\x01\x06\x12\x04\xc0\x02\r\x14\n\x0f\n\x07\
    \x04\x02\x03\x03\x02\x01\x01\x12\x04\xc0\x02\x15\x1c\n\x0f\n\x07\x04\x02\
    \x03\x03\x02\x01\x03\x12\x04\xc0\x02\x1f\x20\n\xb1\x01\n\x04\x04\x02\x03\
    \x04\x12\x06\xc6\x02\x02\xc9\x02\x03\x1a\xa0\x01\x20Declines\x20an\x20in\
    verse\x20offer.\x20Inverse\x20offers\x20should\x20be\x20declined\x20if\n\
    \x20the\x20resources\x20in\x20the\x20offer\x20might\x20not\x20be\x20safe\
    ly\x20evacuated\x20before\n\x20the\x20provided\x20unavailability.\n\n\r\
    \n\x05\x04\x02\x03\x04\x01\x12\x04\xc6\x02\n\x1e\n\x0e\n\x06\x04\x02\x03\
    \x04\x02\0\x12\x04\xc7\x02\x04+\n\x0f\n\x07\x04\x02\x03\x04\x02\0\x04\
    \x12\x04\xc7\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x04\x02\0\x06\x12\x04\
    \xc7\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x04\x02\0\x01\x12\x04\xc7\x02\x15\
    &\n\x0f\n\x07\x04\x02\x03\x04\x02\0\x03\x12\x04\xc7\x02)*\n\x0e\n\x06\
    \x04\x02\x03\x04\x02\x01\x12\x04\xc8\x02\x04!\n\x0f\n\x07\x04\x02\x03\
    \x04\x02\x01\x04\x12\x04\xc8\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x04\x02\
    \x01\x06\x12\x04\xc8\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x04\x02\x01\x01\
    \x12\x04\xc8\x02\x15\x1c\n\x0f\n\x07\x04\x02\x03\x04\x02\x01\x03\x12\x04\
    \xc8\x02\x1f\x20\n\xb4\x01\n\x04\x04\x02\x03\x05\x12\x06\xce\x02\x02\xd0\
    \x02\x03\x1a\xa3\x01\x20Revive\x20offers\x20for\x20the\x20specified\x20r\
    oles.\x20If\x20`roles`\x20is\x20empty,\n\x20the\x20`REVIVE`\x20call\x20w\
    ill\x20revive\x20offers\x20for\x20all\x20of\x20the\x20roles\n\x20the\x20\
    framework\x20is\x20currently\x20subscribed\x20to.\n\n\r\n\x05\x04\x02\
    \x03\x05\x01\x12\x04\xce\x02\n\x10\n\x0e\n\x06\x04\x02\x03\x05\x02\0\x12\
    \x04\xcf\x02\x04\x1e\n\x0f\n\x07\x04\x02\x03\x05\x02\0\x04\x12\x04\xcf\
    \x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x05\x02\0\x05\x12\x04\xcf\x02\r\x13\
    \n\x0f\n\x07\x04\x02\x03\x05\x02\0\x01\x12\x04\xcf\x02\x14\x19\n\x0f\n\
    \x07\x04\x02\x03\x05\x02\0\x03\x12\x04\xcf\x02\x1c\x1d\n\x8f\x06\n\x04\
    \x04\x02\x03\x06\x12\x06\xe0\x02\x02\xe8\x02\x03\x1a\xfe\x05\x20Kills\
    \x20a\x20specific\x20task.\x20If\x20the\x20scheduler\x20has\x20a\x20cust\
    om\x20executor,\n\x20the\x20kill\x20is\x20forwarded\x20to\x20the\x20exec\
    utor\x20and\x20it\x20is\x20up\x20to\x20the\n\x20executor\x20to\x20kill\
    \x20the\x20task\x20and\x20send\x20a\x20TASK_KILLED\x20(or\x20TASK_FAILED\
    )\n\x20update.\x20Note\x20that\x20Mesos\x20releases\x20the\x20resources\
    \x20for\x20a\x20task\x20once\x20it\n\x20receives\x20a\x20terminal\x20upd\
    ate\x20(See\x20TaskState\x20in\x20v1/mesos.proto)\x20for\n\x20it.\x20If\
    \x20the\x20task\x20is\x20unknown\x20to\x20the\x20master,\x20a\x20TASK_LO\
    ST\x20update\x20is\n\x20generated.\n\n\x20If\x20a\x20task\x20within\x20a\
    \x20task\x20group\x20is\x20killed\x20before\x20the\x20group\x20is\n\x20d\
    elivered\x20to\x20the\x20executor,\x20all\x20tasks\x20in\x20the\x20task\
    \x20group\x20are\n\x20killed.\x20When\x20a\x20task\x20group\x20has\x20be\
    en\x20delivered\x20to\x20the\x20executor,\n\x20it\x20is\x20up\x20to\x20t\
    he\x20executor\x20to\x20decide\x20how\x20to\x20deal\x20with\x20the\x20ki\
    ll.\n\x20Note\x20The\x20default\x20Mesos\x20executor\x20will\x20currentl\
    y\x20kill\x20all\x20the\n\x20tasks\x20in\x20the\x20task\x20group\x20if\
    \x20it\x20gets\x20a\x20kill\x20for\x20any\x20task.\n\n\r\n\x05\x04\x02\
    \x03\x06\x01\x12\x04\xe0\x02\n\x0e\n\x0e\n\x06\x04\x02\x03\x06\x02\0\x12\
    \x04\xe1\x02\x04\x20\n\x0f\n\x07\x04\x02\x03\x06\x02\0\x04\x12\x04\xe1\
    \x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x06\x02\0\x06\x12\x04\xe1\x02\r\x13\
    \n\x0f\n\x07\x04\x02\x03\x06\x02\0\x01\x12\x04\xe1\x02\x14\x1b\n\x0f\n\
    \x07\x04\x02\x03\x06\x02\0\x03\x12\x04\xe1\x02\x1e\x1f\n\x0e\n\x06\x04\
    \x02\x03\x06\x02\x01\x12\x04\xe2\x02\x04\"\n\x0f\n\x07\x04\x02\x03\x06\
    \x02\x01\x04\x12\x04\xe2\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x06\x02\x01\
    \x06\x12\x04\xe2\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x06\x02\x01\x01\x12\
    \x04\xe2\x02\x15\x1d\n\x0f\n\x07\x04\x02\x03\x06\x02\x01\x03\x12\x04\xe2\
    \x02\x20!\n\xe5\x01\n\x06\x04\x02\x03\x06\x02\x02\x12\x04\xe7\x02\x04(\
    \x1a\xd4\x01\x20If\x20set,\x20overrides\x20any\x20previously\x20specifie\
    d\x20kill\x20policy\x20for\x20this\x20task.\n\x20This\x20includes\x20'Ta\
    skInfo.kill_policy'\x20and\x20'Executor.kill.kill_policy'.\n\x20Can\x20b\
    e\x20used\x20to\x20forcefully\x20kill\x20a\x20task\x20which\x20is\x20alr\
    eady\x20being\x20killed.\n\n\x0f\n\x07\x04\x02\x03\x06\x02\x02\x04\x12\
    \x04\xe7\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x06\x02\x02\x06\x12\x04\xe7\
    \x02\r\x17\n\x0f\n\x07\x04\x02\x03\x06\x02\x02\x01\x12\x04\xe7\x02\x18#\
    \n\x0f\n\x07\x04\x02\x03\x06\x02\x02\x03\x12\x04\xe7\x02&'\n\xa8\x03\n\
    \x04\x04\x02\x03\x07\x12\x06\xf1\x02\x02\xf4\x02\x03\x1a\x97\x03\x20Shut\
    s\x20down\x20a\x20custom\x20executor.\x20When\x20the\x20executor\x20gets\
    \x20a\x20shutdown\n\x20event,\x20it\x20is\x20expected\x20to\x20kill\x20a\
    ll\x20its\x20tasks\x20(and\x20send\x20TASK_KILLED\n\x20updates)\x20and\
    \x20terminate.\x20If\x20the\x20executor\x20doesn\xe2\x80\x99t\x20termina\
    te\x20within\n\x20a\x20certain\x20timeout\x20(configurable\x20via\n\x20'\
    --executor_shutdown_grace_period'\x20agent\x20flag),\x20the\x20agent\x20\
    will\n\x20forcefully\x20destroy\x20the\x20container\x20(executor\x20and\
    \x20its\x20tasks)\x20and\n\x20transition\x20its\x20active\x20tasks\x20to\
    \x20TASK_LOST.\n\n\r\n\x05\x04\x02\x03\x07\x01\x12\x04\xf1\x02\n\x12\n\
    \x0e\n\x06\x04\x02\x03\x07\x02\0\x12\x04\xf2\x02\x04(\n\x0f\n\x07\x04\
    \x02\x03\x07\x02\0\x04\x12\x04\xf2\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\
    \x07\x02\0\x06\x12\x04\xf2\x02\r\x17\n\x0f\n\x07\x04\x02\x03\x07\x02\0\
    \x01\x12\x04\xf2\x02\x18#\n\x0f\n\x07\x04\x02\x03\x07\x02\0\x03\x12\x04\
    \xf2\x02&'\n\x0e\n\x06\x04\x02\x03\x07\x02\x01\x12\x04\xf3\x02\x04\"\n\
    \x0f\n\x07\x04\x02\x03\x07\x02\x01\x04\x12\x04\xf3\x02\x04\x0c\n\x0f\n\
    \x07\x04\x02\x03\x07\x02\x01\x06\x12\x04\xf3\x02\r\x14\n\x0f\n\x07\x04\
    \x02\x03\x07\x02\x01\x01\x12\x04\xf3\x02\x15\x1d\n\x0f\n\x07\x04\x02\x03\
    \x07\x02\x01\x03\x12\x04\xf3\x02\x20!\n\xa0\x02\n\x04\x04\x02\x03\x08\
    \x12\x06\xfb\x02\x02\xff\x02\x03\x1a\x8f\x02\x20Acknowledges\x20the\x20r\
    eceipt\x20of\x20status\x20update.\x20Schedulers\x20are\n\x20responsible\
    \x20for\x20explicitly\x20acknowledging\x20the\x20receipt\x20of\x20status\
    \n\x20updates\x20that\x20have\x20'Update.status().uuid()'\x20field\x20se\
    t.\x20Such\x20status\n\x20updates\x20are\x20retried\x20by\x20the\x20agen\
    t\x20until\x20they\x20are\x20acknowledged\x20by\n\x20the\x20scheduler.\n\
    \n\r\n\x05\x04\x02\x03\x08\x01\x12\x04\xfb\x02\n\x15\n\x0e\n\x06\x04\x02\
    \x03\x08\x02\0\x12\x04\xfc\x02\x04\"\n\x0f\n\x07\x04\x02\x03\x08\x02\0\
    \x04\x12\x04\xfc\x02\x04\x0c\n\x0f\n\x07\x04\x02\x03\x08\x02\0\x06\x12\
    \x04\xfc\x02\r\x14\n\x0f\n\x07\x04\x02\x03\x08\x02\0\x01\x12\x04\xfc\x02\
    \x15\x1d\n\x0f\n\x07\x04\x02\x03\x08\x02\0\x03\x12\x04\xfc\x02\x20!\n\
    \x0e\n\x06\x04\x02\x03\x08\x02\x01\x12\x04\xfd\x02\x04\x20\n\x0f\n\x07\
    \x04\x02\x03\x08\x02\x01\x04\x12\x04\xfd\x02\x04\x0c\n\x0f\n\x07\x04\x02\
    \x03\x08\x02\x01\x06\x12\x04\xfd\x02\r\x13\n\x0f\n\x07\x04\x02\x03\x08\
    \x02\x01\x01\x12\x04\xfd\x02\x14\x1b\n\x0f\n\x07\x04\x02\x03\x08\x02\x01\
    \x03\x12\x04\xfd\x02\x1e\x1f\n\x0e\n\x06\x04\x02\x03\x08\x02\x02\x12\x04\
    \xfe\x02\x04\x1c\n\x0f\n\x07\x04\x02\x03\x08\x02\x02\x04\x12\x04\xfe\x02\
    \x04\x0c\n\x0f\n\x07\x04\x02\x03\x08\x02\x02\x05\x12\x04\xfe\x02\r\x12\n\
    \x0f\n\x07\x04\x02\x03\x08\x02\x02\x01\x12\x04\xfe\x02\x13\x17\n\x0f\n\
    \x07\x04\x02\x03\x08\x02\x02\x03\x12\x04\xfe\x02\x1a\x1b\n\xcf\x02\n\x04\
    \x04\x02\x03\t\x12\x06\x86\x03\x02\x91\x03\x03\x1a\xbe\x02\x20Acknowledg\
    es\x20the\x20receipt\x20of\x20an\x20operation\x20status\x20update.\x20Sc\
    hedulers\n\x20are\x20responsible\x20for\x20explicitly\x20acknowledging\
    \x20the\x20receipt\x20of\x20updates\n\x20which\x20have\x20the\x20'Update\
    OperationStatus.status().uuid()'\x20field\x20set.\n\x20Such\x20status\
    \x20updates\x20are\x20retried\x20by\x20the\x20agent\x20or\x20resource\
    \x20provider\n\x20until\x20they\x20are\x20acknowledged\x20by\x20the\x20s\
    cheduler.\n\n\r\n\x05\x04\x02\x03\t\x01\x12\x04\x86\x03\n$\n\xf0\x01\n\
    \x06\x04\x02\x03\t\x02\0\x12\x04\x8c\x03\x04\"\x1a\xdf\x01\x20If\x20the\
    \x20operation\x20affects\x20resources\x20that\x20belong\x20to\x20a\x20SL\
    RP,\x20both\n\x20`agent_id`\x20and\x20`resource_provider_id`\x20have\x20\
    to\x20be\x20set.\n\n\x20If\x20the\x20operation\x20affects\x20resources\
    \x20that\x20belong\x20to\x20a\x20SERP,\x20only\n\x20`resource_provider_i\
    d`\x20has\x20to\x20be\x20set.\n\n\x0f\n\x07\x04\x02\x03\t\x02\0\x04\x12\
    \x04\x8c\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\t\x02\0\x06\x12\x04\x8c\x03\
    \r\x14\n\x0f\n\x07\x04\x02\x03\t\x02\0\x01\x12\x04\x8c\x03\x15\x1d\n\x0f\
    \n\x07\x04\x02\x03\t\x02\0\x03\x12\x04\x8c\x03\x20!\n\x0e\n\x06\x04\x02\
    \x03\t\x02\x01\x12\x04\x8d\x03\x049\n\x0f\n\x07\x04\x02\x03\t\x02\x01\
    \x04\x12\x04\x8d\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\t\x02\x01\x06\x12\
    \x04\x8d\x03\r\x1f\n\x0f\n\x07\x04\x02\x03\t\x02\x01\x01\x12\x04\x8d\x03\
    \x204\n\x0f\n\x07\x04\x02\x03\t\x02\x01\x03\x12\x04\x8d\x0378\n\x0e\n\
    \x06\x04\x02\x03\t\x02\x02\x12\x04\x8f\x03\x04\x1c\n\x0f\n\x07\x04\x02\
    \x03\t\x02\x02\x04\x12\x04\x8f\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\t\x02\
    \x02\x05\x12\x04\x8f\x03\r\x12\n\x0f\n\x07\x04\x02\x03\t\x02\x02\x01\x12\
    \x04\x8f\x03\x13\x17\n\x0f\n\x07\x04\x02\x03\t\x02\x02\x03\x12\x04\x8f\
    \x03\x1a\x1b\n\x0e\n\x06\x04\x02\x03\t\x02\x03\x12\x04\x90\x03\x04*\n\
    \x0f\n\x07\x04\x02\x03\t\x02\x03\x04\x12\x04\x90\x03\x04\x0c\n\x0f\n\x07\
    \x04\x02\x03\t\x02\x03\x06\x12\x04\x90\x03\r\x18\n\x0f\n\x07\x04\x02\x03\
    \t\x02\x03\x01\x12\x04\x90\x03\x19%\n\x0f\n\x07\x04\x02\x03\t\x02\x03\
    \x03\x12\x04\x90\x03()\n\xff\x02\n\x04\x04\x02\x03\n\x12\x06\x99\x03\x02\
    \xa1\x03\x03\x1a\xee\x02\x20Allows\x20the\x20scheduler\x20to\x20query\
    \x20the\x20status\x20for\x20non-terminal\x20tasks.\n\x20This\x20causes\
    \x20the\x20master\x20to\x20send\x20back\x20the\x20latest\x20task\x20stat\
    us\x20for\n\x20each\x20task\x20in\x20'tasks',\x20if\x20possible.\x20Task\
    s\x20that\x20are\x20no\x20longer\x20known\n\x20will\x20result\x20in\x20a\
    \x20TASK_LOST,\x20TASK_UNKNOWN,\x20or\x20TASK_UNREACHABLE\x20update.\n\
    \x20If\x20'tasks'\x20is\x20empty,\x20then\x20the\x20master\x20will\x20se\
    nd\x20the\x20latest\x20status\n\x20for\x20each\x20task\x20currently\x20k\
    nown.\n\n\r\n\x05\x04\x02\x03\n\x01\x12\x04\x99\x03\n\x13\nT\n\x06\x04\
    \x02\x03\n\x03\0\x12\x06\x9b\x03\x04\x9e\x03\x05\x1aB\x20TODO(vinod):\
    \x20Support\x20arbitrary\x20queries\x20than\x20just\x20state\x20of\x20ta\
    sks.\n\n\x0f\n\x07\x04\x02\x03\n\x03\0\x01\x12\x04\x9b\x03\x0c\x10\n\x10\
    \n\x08\x04\x02\x03\n\x03\0\x02\0\x12\x04\x9c\x03\x06\"\n\x11\n\t\x04\x02\
    \x03\n\x03\0\x02\0\x04\x12\x04\x9c\x03\x06\x0e\n\x11\n\t\x04\x02\x03\n\
    \x03\0\x02\0\x06\x12\x04\x9c\x03\x0f\x15\n\x11\n\t\x04\x02\x03\n\x03\0\
    \x02\0\x01\x12\x04\x9c\x03\x16\x1d\n\x11\n\t\x04\x02\x03\n\x03\0\x02\0\
    \x03\x12\x04\x9c\x03\x20!\n\x10\n\x08\x04\x02\x03\n\x03\0\x02\x01\x12\
    \x04\x9d\x03\x06$\n\x11\n\t\x04\x02\x03\n\x03\0\x02\x01\x04\x12\x04\x9d\
    \x03\x06\x0e\n\x11\n\t\x04\x02\x03\n\x03\0\x02\x01\x06\x12\x04\x9d\x03\
    \x0f\x16\n\x11\n\t\x04\x02\x03\n\x03\0\x02\x01\x01\x12\x04\x9d\x03\x17\
    \x1f\n\x11\n\t\x04\x02\x03\n\x03\0\x02\x01\x03\x12\x04\x9d\x03\"#\n\x0e\
    \n\x06\x04\x02\x03\n\x02\0\x12\x04\xa0\x03\x04\x1c\n\x0f\n\x07\x04\x02\
    \x03\n\x02\0\x04\x12\x04\xa0\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\n\x02\0\
    \x06\x12\x04\xa0\x03\r\x11\n\x0f\n\x07\x04\x02\x03\n\x02\0\x01\x12\x04\
    \xa0\x03\x12\x17\n\x0f\n\x07\x04\x02\x03\n\x02\0\x03\x12\x04\xa0\x03\x1a\
    \x1b\n\x9f\x02\n\x04\x04\x02\x03\x0b\x12\x06\xa8\x03\x02\xb0\x03\x03\x1a\
    \x8e\x02\x20Allows\x20the\x20scheduler\x20to\x20query\x20the\x20status\
    \x20of\x20operations.\x20This\x20causes\n\x20the\x20master\x20to\x20send\
    \x20back\x20the\x20latest\x20status\x20for\x20each\x20operation\x20in\n\
    \x20'operations',\x20if\x20possible.\x20If\x20'operations'\x20is\x20empt\
    y,\x20then\x20the\n\x20master\x20will\x20send\x20the\x20latest\x20status\
    \x20for\x20each\x20operation\x20currently\n\x20known.\n\n\r\n\x05\x04\
    \x02\x03\x0b\x01\x12\x04\xa8\x03\n\x1d\n\x10\n\x06\x04\x02\x03\x0b\x03\0\
    \x12\x06\xa9\x03\x04\xad\x03\x05\n\x0f\n\x07\x04\x02\x03\x0b\x03\0\x01\
    \x12\x04\xa9\x03\x0c\x15\n\x10\n\x08\x04\x02\x03\x0b\x03\0\x02\0\x12\x04\
    \xaa\x03\x06,\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\0\x04\x12\x04\xaa\x03\
    \x06\x0e\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\0\x06\x12\x04\xaa\x03\x0f\
    \x1a\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\0\x01\x12\x04\xaa\x03\x1b'\n\
    \x11\n\t\x04\x02\x03\x0b\x03\0\x02\0\x03\x12\x04\xaa\x03*+\n\x10\n\x08\
    \x04\x02\x03\x0b\x03\0\x02\x01\x12\x04\xab\x03\x06$\n\x11\n\t\x04\x02\
    \x03\x0b\x03\0\x02\x01\x04\x12\x04\xab\x03\x06\x0e\n\x11\n\t\x04\x02\x03\
    \x0b\x03\0\x02\x01\x06\x12\x04\xab\x03\x0f\x16\n\x11\n\t\x04\x02\x03\x0b\
    \x03\0\x02\x01\x01\x12\x04\xab\x03\x17\x1f\n\x11\n\t\x04\x02\x03\x0b\x03\
    \0\x02\x01\x03\x12\x04\xab\x03\"#\n\x10\n\x08\x04\x02\x03\x0b\x03\0\x02\
    \x02\x12\x04\xac\x03\x06;\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\x02\x04\
    \x12\x04\xac\x03\x06\x0e\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\x02\x06\x12\
    \x04\xac\x03\x0f!\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\x02\x01\x12\x04\
    \xac\x03\"6\n\x11\n\t\x04\x02\x03\x0b\x03\0\x02\x02\x03\x12\x04\xac\x039\
    :\n\x0e\n\x06\x04\x02\x03\x0b\x02\0\x12\x04\xaf\x03\x04&\n\x0f\n\x07\x04\
    \x02\x03\x0b\x02\0\x04\x12\x04\xaf\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\
    \x0b\x02\0\x06\x12\x04\xaf\x03\r\x16\n\x0f\n\x07\x04\x02\x03\x0b\x02\0\
    \x01\x12\x04\xaf\x03\x17!\n\x0f\n\x07\x04\x02\x03\x0b\x02\0\x03\x12\x04\
    \xaf\x03$%\n\xbb\x01\n\x04\x04\x02\x03\x0c\x12\x06\xb5\x03\x02\xb9\x03\
    \x03\x1a\xaa\x01\x20Sends\x20arbitrary\x20binary\x20data\x20to\x20the\
    \x20executor.\x20Note\x20that\x20Mesos\n\x20neither\x20interprets\x20thi\
    s\x20data\x20nor\x20makes\x20any\x20guarantees\x20about\x20the\n\x20deli\
    very\x20of\x20this\x20message\x20to\x20the\x20executor.\n\n\r\n\x05\x04\
    \x02\x03\x0c\x01\x12\x04\xb5\x03\n\x11\n\x0e\n\x06\x04\x02\x03\x0c\x02\0\
    \x12\x04\xb6\x03\x04\"\n\x0f\n\x07\x04\x02\x03\x0c\x02\0\x04\x12\x04\xb6\
    \x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\x0c\x02\0\x06\x12\x04\xb6\x03\r\x14\
    \n\x0f\n\x07\x04\x02\x03\x0c\x02\0\x01\x12\x04\xb6\x03\x15\x1d\n\x0f\n\
    \x07\x04\x02\x03\x0c\x02\0\x03\x12\x04\xb6\x03\x20!\n\x0e\n\x06\x04\x02\
    \x03\x0c\x02\x01\x12\x04\xb7\x03\x04(\n\x0f\n\x07\x04\x02\x03\x0c\x02\
    \x01\x04\x12\x04\xb7\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\x0c\x02\x01\x06\
    \x12\x04\xb7\x03\r\x17\n\x0f\n\x07\x04\x02\x03\x0c\x02\x01\x01\x12\x04\
    \xb7\x03\x18#\n\x0f\n\x07\x04\x02\x03\x0c\x02\x01\x03\x12\x04\xb7\x03&'\
    \n\x0e\n\x06\x04\x02\x03\x0c\x02\x02\x12\x04\xb8\x03\x04\x1c\n\x0f\n\x07\
    \x04\x02\x03\x0c\x02\x02\x04\x12\x04\xb8\x03\x04\x0c\n\x0f\n\x07\x04\x02\
    \x03\x0c\x02\x02\x05\x12\x04\xb8\x03\r\x12\n\x0f\n\x07\x04\x02\x03\x0c\
    \x02\x02\x01\x12\x04\xb8\x03\x13\x17\n\x0f\n\x07\x04\x02\x03\x0c\x02\x02\
    \x03\x12\x04\xb8\x03\x1a\x1b\n\xaf\x02\n\x04\x04\x02\x03\r\x12\x06\xc1\
    \x03\x02\xc3\x03\x03\x1a\x9e\x02\x20Requests\x20a\x20specific\x20set\x20\
    of\x20resources\x20from\x20Mesos's\x20allocator.\x20If\n\x20the\x20alloc\
    ator\x20has\x20support\x20for\x20this,\x20corresponding\x20offers\x20wil\
    l\x20be\n\x20sent\x20asynchronously\x20via\x20the\x20OFFERS\x20event(s).\
    \n\n\x20NOTE:\x20The\x20built-in\x20hierarchical\x20allocator\x20doesn't\
    \x20have\x20support\n\x20for\x20this\x20call\x20and\x20hence\x20simply\
    \x20ignores\x20it.\n\n\r\n\x05\x04\x02\x03\r\x01\x12\x04\xc1\x03\n\x11\n\
    \x0e\n\x06\x04\x02\x03\r\x02\0\x12\x04\xc2\x03\x04+\n\x0f\n\x07\x04\x02\
    \x03\r\x02\0\x04\x12\x04\xc2\x03\x04\x0c\n\x0f\n\x07\x04\x02\x03\r\x02\0\
    \x06\x12\x04\xc2\x03\r\x1d\n\x0f\n\x07\x04\x02\x03\r\x02\0\x01\x12\x04\
    \xc2\x03\x1e&\n\x0f\n\x07\x04\x02\x03\r\x02\0\x03\x12\x04\xc2\x03)*\n\
    \xba\x01\n\x04\x04\x02\x03\x0e\x12\x06\xc8\x03\x02\xca\x03\x03\x1a\xa9\
    \x01\x20Suppress\x20offers\x20for\x20the\x20specified\x20roles.\x20If\
    \x20`roles`\x20is\x20empty,\n\x20the\x20`SUPPRESS`\x20call\x20will\x20su\
    ppress\x20offers\x20for\x20all\x20of\x20the\x20roles\n\x20the\x20framewo\
    rk\x20is\x20currently\x20subscribed\x20to.\n\n\r\n\x05\x04\x02\x03\x0e\
    \x01\x12\x04\xc8\x03\n\x12\n\x0e\n\x06\x04\x02\x03\x0e\x02\0\x12\x04\xc9\
    \x03\x04\x1e\n\x0f\n\x07\x04\x02\x03\x0e\x02\0\x04\x12\x04\xc9\x03\x04\
    \x0c\n\x0f\n\x07\x04\x02\x03\x0e\x02\0\x05\x12\x04\xc9\x03\r\x13\n\x0f\n\
    \x07\x04\x02\x03\x0e\x02\0\x01\x12\x04\xc9\x03\x14\x19\n\x0f\n\x07\x04\
    \x02\x03\x0e\x02\0\x03\x12\x04\xc9\x03\x1c\x1d\n\xea\x02\n\x04\x04\x02\
    \x02\0\x12\x04\xd2\x03\x02(\x1a\xdb\x02\x20Identifies\x20who\x20generate\
    d\x20this\x20call.\x20Master\x20assigns\x20a\x20framework\x20id\n\x20whe\
    n\x20a\x20new\x20scheduler\x20subscribes\x20for\x20the\x20first\x20time.\
    \x20Once\x20assigned,\n\x20the\x20scheduler\x20must\x20set\x20the\x20'fr\
    amework_id'\x20here\x20and\x20within\x20its\n\x20FrameworkInfo\x20(in\
    \x20any\x20further\x20'Subscribe'\x20calls).\x20This\x20allows\x20the\n\
    \x20master\x20to\x20identify\x20a\x20scheduler\x20correctly\x20across\
    \x20disconnections,\n\x20failovers,\x20etc.\n\n\r\n\x05\x04\x02\x02\0\
    \x04\x12\x04\xd2\x03\x02\n\n\r\n\x05\x04\x02\x02\0\x06\x12\x04\xd2\x03\
    \x0b\x16\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\xd2\x03\x17#\n\r\n\x05\x04\
    \x02\x02\0\x03\x12\x04\xd2\x03&'\n\xe1\x01\n\x04\x04\x02\x02\x01\x12\x04\
    \xd8\x03\x02\x19\x1a\xd2\x01\x20Type\x20of\x20the\x20call,\x20indicates\
    \x20which\x20optional\x20field\x20below\x20should\x20be\n\x20present\x20\
    if\x20that\x20type\x20has\x20a\x20nested\x20message\x20definition.\n\x20\
    See\x20comments\x20on\x20`Event::Type`\x20above\x20on\x20the\x20reasonin\
    g\x20behind\x20this\n\x20field\x20being\x20optional.\n\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x04\xd8\x03\x02\n\n\r\n\x05\x04\x02\x02\x01\x06\x12\x04\
    \xd8\x03\x0b\x0f\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\xd8\x03\x10\x14\n\
    \r\n\x05\x04\x02\x02\x01\x03\x12\x04\xd8\x03\x17\x18\n\x0c\n\x04\x04\x02\
    \x02\x02\x12\x04\xda\x03\x02#\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\xda\
    \x03\x02\n\n\r\n\x05\x04\x02\x02\x02\x06\x12\x04\xda\x03\x0b\x14\n\r\n\
    \x05\x04\x02\x02\x02\x01\x12\x04\xda\x03\x15\x1e\n\r\n\x05\x04\x02\x02\
    \x02\x03\x12\x04\xda\x03!\"\n\x0c\n\x04\x04\x02\x02\x03\x12\x04\xdb\x03\
    \x02\x1d\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04\xdb\x03\x02\n\n\r\n\x05\
    \x04\x02\x02\x03\x06\x12\x04\xdb\x03\x0b\x11\n\r\n\x05\x04\x02\x02\x03\
    \x01\x12\x04\xdb\x03\x12\x18\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\xdb\
    \x03\x1b\x1c\n\x0c\n\x04\x04\x02\x02\x04\x12\x04\xdc\x03\x02\x1f\n\r\n\
    \x05\x04\x02\x02\x04\x04\x12\x04\xdc\x03\x02\n\n\r\n\x05\x04\x02\x02\x04\
    \x06\x12\x04\xdc\x03\x0b\x12\n\r\n\x05\x04\x02\x02\x04\x01\x12\x04\xdc\
    \x03\x13\x1a\n\r\n\x05\x04\x02\x02\x04\x03\x12\x04\xdc\x03\x1d\x1e\n\x0c\
    \n\x04\x04\x02\x02\x05\x12\x04\xdd\x03\x02:\n\r\n\x05\x04\x02\x02\x05\
    \x04\x12\x04\xdd\x03\x02\n\n\r\n\x05\x04\x02\x02\x05\x06\x12\x04\xdd\x03\
    \x0b\x1e\n\r\n\x05\x04\x02\x02\x05\x01\x12\x04\xdd\x03\x1f4\n\r\n\x05\
    \x04\x02\x02\x05\x03\x12\x04\xdd\x0379\n\x0c\n\x04\x04\x02\x02\x06\x12\
    \x04\xde\x03\x02<\n\r\n\x05\x04\x02\x02\x06\x04\x12\x04\xde\x03\x02\n\n\
    \r\n\x05\x04\x02\x02\x06\x06\x12\x04\xde\x03\x0b\x1f\n\r\n\x05\x04\x02\
    \x02\x06\x01\x12\x04\xde\x03\x206\n\r\n\x05\x04\x02\x02\x06\x03\x12\x04\
    \xde\x039;\n\x0c\n\x04\x04\x02\x02\x07\x12\x04\xdf\x03\x02\x1e\n\r\n\x05\
    \x04\x02\x02\x07\x04\x12\x04\xdf\x03\x02\n\n\r\n\x05\x04\x02\x02\x07\x06\
    \x12\x04\xdf\x03\x0b\x11\n\r\n\x05\x04\x02\x02\x07\x01\x12\x04\xdf\x03\
    \x12\x18\n\r\n\x05\x04\x02\x02\x07\x03\x12\x04\xdf\x03\x1b\x1d\n\x0c\n\
    \x04\x04\x02\x02\x08\x12\x04\xe0\x03\x02\x19\n\r\n\x05\x04\x02\x02\x08\
    \x04\x12\x04\xe0\x03\x02\n\n\r\n\x05\x04\x02\x02\x08\x06\x12\x04\xe0\x03\
    \x0b\x0f\n\r\n\x05\x04\x02\x02\x08\x01\x12\x04\xe0\x03\x10\x14\n\r\n\x05\
    \x04\x02\x02\x08\x03\x12\x04\xe0\x03\x17\x18\n\x0c\n\x04\x04\x02\x02\t\
    \x12\x04\xe1\x03\x02!\n\r\n\x05\x04\x02\x02\t\x04\x12\x04\xe1\x03\x02\n\
    \n\r\n\x05\x04\x02\x02\t\x06\x12\x04\xe1\x03\x0b\x13\n\r\n\x05\x04\x02\
    \x02\t\x01\x12\x04\xe1\x03\x14\x1c\n\r\n\x05\x04\x02\x02\t\x03\x12\x04\
    \xe1\x03\x1f\x20\n\x0c\n\x04\x04\x02\x02\n\x12\x04\xe2\x03\x02'\n\r\n\
    \x05\x04\x02\x02\n\x04\x12\x04\xe2\x03\x02\n\n\r\n\x05\x04\x02\x02\n\x06\
    \x12\x04\xe2\x03\x0b\x16\n\r\n\x05\x04\x02\x02\n\x01\x12\x04\xe2\x03\x17\
    \"\n\r\n\x05\x04\x02\x02\n\x03\x12\x04\xe2\x03%&\n\x0c\n\x04\x04\x02\x02\
    \x0b\x12\x04\xe3\x03\x02H\n\r\n\x05\x04\x02\x02\x0b\x04\x12\x04\xe3\x03\
    \x02\n\n\r\n\x05\x04\x02\x02\x0b\x06\x12\x04\xe3\x03\x0b%\n\r\n\x05\x04\
    \x02\x02\x0b\x01\x12\x04\xe3\x03&B\n\r\n\x05\x04\x02\x02\x0b\x03\x12\x04\
    \xe3\x03EG\n\x0c\n\x04\x04\x02\x02\x0c\x12\x04\xe4\x03\x02#\n\r\n\x05\
    \x04\x02\x02\x0c\x04\x12\x04\xe4\x03\x02\n\n\r\n\x05\x04\x02\x02\x0c\x06\
    \x12\x04\xe4\x03\x0b\x14\n\r\n\x05\x04\x02\x02\x0c\x01\x12\x04\xe4\x03\
    \x15\x1e\n\r\n\x05\x04\x02\x02\x0c\x03\x12\x04\xe4\x03!\"\n\x0c\n\x04\
    \x04\x02\x02\r\x12\x04\xe5\x03\x029\n\r\n\x05\x04\x02\x02\r\x04\x12\x04\
    \xe5\x03\x02\n\n\r\n\x05\x04\x02\x02\r\x06\x12\x04\xe5\x03\x0b\x1e\n\r\n\
    \x05\x04\x02\x02\r\x01\x12\x04\xe5\x03\x1f3\n\r\n\x05\x04\x02\x02\r\x03\
    \x12\x04\xe5\x0368\n\x0c\n\x04\x04\x02\x02\x0e\x12\x04\xe6\x03\x02\x20\n\
    \r\n\x05\x04\x02\x02\x0e\x04\x12\x04\xe6\x03\x02\n\n\r\n\x05\x04\x02\x02\
    \x0e\x06\x12\x04\xe6\x03\x0b\x12\n\r\n\x05\x04\x02\x02\x0e\x01\x12\x04\
    \xe6\x03\x13\x1a\n\r\n\x05\x04\x02\x02\x0e\x03\x12\x04\xe6\x03\x1d\x1f\n\
    \x0c\n\x04\x04\x02\x02\x0f\x12\x04\xe7\x03\x02\x20\n\r\n\x05\x04\x02\x02\
    \x0f\x04\x12\x04\xe7\x03\x02\n\n\r\n\x05\x04\x02\x02\x0f\x06\x12\x04\xe7\
    \x03\x0b\x12\n\r\n\x05\x04\x02\x02\x0f\x01\x12\x04\xe7\x03\x13\x1a\n\r\n\
    \x05\x04\x02\x02\x0f\x03\x12\x04\xe7\x03\x1d\x1f\n\x0c\n\x04\x04\x02\x02\
    \x10\x12\x04\xe8\x03\x02\"\n\r\n\x05\x04\x02\x02\x10\x04\x12\x04\xe8\x03\
    \x02\n\n\r\n\x05\x04\x02\x02\x10\x06\x12\x04\xe8\x03\x0b\x13\n\r\n\x05\
    \x04\x02\x02\x10\x01\x12\x04\xe8\x03\x14\x1c\n\r\n\x05\x04\x02\x02\x10\
    \x03\x12\x04\xe8\x03\x1f!\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
