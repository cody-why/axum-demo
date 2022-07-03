/*** 
 * @Author: plucky
 * @Date: 2022-07-01 15:16:39
 * @LastEditTime: 2022-07-01 17:19:34
 * @Description: 
 */

use tower::Service;

#[derive(Clone)]
pub struct HelloMiddleware<S>{
    inner:S
}

//impl  Service<Request<ResBody>> for  HelloMiddleware<S>{}
    
