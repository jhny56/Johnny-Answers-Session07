// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from my_interfaces:srv/FindWall.idl
// generated code does not contain a copyright notice

#ifndef MY_INTERFACES__SRV__DETAIL__FIND_WALL__BUILDER_HPP_
#define MY_INTERFACES__SRV__DETAIL__FIND_WALL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "my_interfaces/srv/detail/find_wall__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace my_interfaces
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_interfaces::srv::FindWall_Request>()
{
  return ::my_interfaces::srv::FindWall_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace my_interfaces


namespace my_interfaces
{

namespace srv
{

namespace builder
{

class Init_FindWall_Response_success
{
public:
  Init_FindWall_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::my_interfaces::srv::FindWall_Response success(::my_interfaces::srv::FindWall_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::my_interfaces::srv::FindWall_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_interfaces::srv::FindWall_Response>()
{
  return my_interfaces::srv::builder::Init_FindWall_Response_success();
}

}  // namespace my_interfaces

#endif  // MY_INTERFACES__SRV__DETAIL__FIND_WALL__BUILDER_HPP_
