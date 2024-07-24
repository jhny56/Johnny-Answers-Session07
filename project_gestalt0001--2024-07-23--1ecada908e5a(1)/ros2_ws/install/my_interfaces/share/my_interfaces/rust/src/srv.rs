

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FindWall_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for FindWall_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::FindWall_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FindWall_Request {
  type RmwMsg = crate::srv::rmw::FindWall_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FindWall_Response {
    pub success: bool,
}



impl Default for FindWall_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::FindWall_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FindWall_Response {
  type RmwMsg = crate::srv::rmw::FindWall_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}






#[link(name = "my_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_interfaces__srv__FindWall() -> *const std::os::raw::c_void;
}

// Corresponds to my_interfaces__srv__FindWall
pub struct FindWall;

impl rosidl_runtime_rs::Service for FindWall {
  type Request = crate::srv::FindWall_Request;
  type Response = crate::srv::FindWall_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_interfaces__srv__FindWall() }
  }
}



pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "my_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_interfaces__srv__FindWall_Request() -> *const std::os::raw::c_void;
}

#[link(name = "my_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_interfaces__srv__FindWall_Request__init(msg: *mut FindWall_Request) -> bool;
    fn my_interfaces__srv__FindWall_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FindWall_Request>, size: usize) -> bool;
    fn my_interfaces__srv__FindWall_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FindWall_Request>);
    fn my_interfaces__srv__FindWall_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FindWall_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FindWall_Request>) -> bool;
}

// Corresponds to my_interfaces__srv__FindWall_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FindWall_Request {
    pub structure_needs_at_least_one_member: u8,
}



impl Default for FindWall_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_interfaces__srv__FindWall_Request__init(&mut msg as *mut _) {
        panic!("Call to my_interfaces__srv__FindWall_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FindWall_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_interfaces__srv__FindWall_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_interfaces__srv__FindWall_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_interfaces__srv__FindWall_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FindWall_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FindWall_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_interfaces/srv/FindWall_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_interfaces__srv__FindWall_Request() }
  }
}


#[link(name = "my_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_interfaces__srv__FindWall_Response() -> *const std::os::raw::c_void;
}

#[link(name = "my_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_interfaces__srv__FindWall_Response__init(msg: *mut FindWall_Response) -> bool;
    fn my_interfaces__srv__FindWall_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FindWall_Response>, size: usize) -> bool;
    fn my_interfaces__srv__FindWall_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FindWall_Response>);
    fn my_interfaces__srv__FindWall_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FindWall_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FindWall_Response>) -> bool;
}

// Corresponds to my_interfaces__srv__FindWall_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FindWall_Response {
    pub success: bool,
}



impl Default for FindWall_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_interfaces__srv__FindWall_Response__init(&mut msg as *mut _) {
        panic!("Call to my_interfaces__srv__FindWall_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FindWall_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_interfaces__srv__FindWall_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_interfaces__srv__FindWall_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_interfaces__srv__FindWall_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FindWall_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FindWall_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_interfaces/srv/FindWall_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_interfaces__srv__FindWall_Response() }
  }
}






  #[link(name = "my_interfaces__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__my_interfaces__srv__FindWall() -> *const std::os::raw::c_void;
  }

  // Corresponds to my_interfaces__srv__FindWall
  pub struct FindWall;

  impl rosidl_runtime_rs::Service for FindWall {
    type Request = crate::srv::rmw::FindWall_Request;
    type Response = crate::srv::rmw::FindWall_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_interfaces__srv__FindWall() }
    }
  }



}  // mod rmw
