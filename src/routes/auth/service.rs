/*
 * @Author: plucky
 * @Date: 2023-10-27 11:18:09
 * @LastEditTime: 2023-10-27 11:18:40
 */

use super::dtos::create_hello_request::CreateHelloRequest;
use super::dtos::create_hello_response::CreateHelloResponse;
use super::dtos::get_hello_response::GetHelloResponse;
use super::dtos::list_hello_response::ListHelloResponse;
use super::dtos::update_hello_request::UpdateHelloRequest;
use super::dtos::update_hello_response::UpdateHelloResponse;
use super::dtos::delete_hello_response::DeleteHelloResponse;

use super::vo::hello::Hello;

pub struct HelloService {}

impl HelloService {
    pub fn new() -> Self {
        Self {}
    }

    
}
