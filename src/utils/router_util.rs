/*
 * @Author: plucky
 * @Date: 2023-10-27 21:38:33
 * @LastEditTime: 2023-10-29 17:28:54
 */
#![allow(unused_imports)]

#[test]
fn test_route_debug() {
    use std::collections::HashMap;
    use regex::Regex;
    
    let app = crate::routes::root::app();
    // println!("{:#?}",app);
    let msg = format!("{:?}",app);
    // println!("{:#?}",msg);
    
    //查找 "Node { paths:"和 "} } }," 之间的内容
    let start = msg.find("Node { paths:").unwrap()+ 15;
    let end = msg.find("} } },").unwrap();
    let paths = msg[start..end].replace('\"', "");
    println!("{:?}", paths);

    // 用,分割再用:分割,变成Map
    let map1: HashMap<_,_> = paths.split(',').map(|x| {
        let mut s = x.split(':');
        (s.next().unwrap().trim(), s.next().unwrap().trim())
    }).collect();

    println!("{:#?}",map1);

    // 用regex提取RouteId(数字)和方法
    let reg = r"RouteId\((?P<route_id>\d+)\): MethodRouter\(MethodRouter \{(?P<methods>[^\}]+)\}";
    let re = Regex::new(reg).unwrap();
    // Bytes(b"和")之间的内容
    let re2 = Regex::new(r#"Bytes\(b"(.*?)"\)"#).unwrap();


    let map2 = re.captures_iter(&msg).map(|x: regex::Captures<'_>| {
        let route_id = x.name("route_id").map(|x|{format!("RouteId({})",x.as_str())}).unwrap_or_default();
        let methods = x.name("methods").map(|x|{x.as_str()}).unwrap_or("");
        let methods = re2.captures(methods).unwrap().get(1).unwrap().as_str().trim_end_matches(",HEAD");
        (route_id, methods)
    }).collect::<HashMap<_,_>>();
    println!("{:#?}", map2);

    // 把map1和map2合并,map1的value加map2的value组成新map,输出 "RouteId(1)": "GET /"
    let map3 = map1.into_iter().map(|(k,v)| (k, map2.get(k).unwrap_or(&"").to_string()+" "+v)).collect::<HashMap<_,_>>();
    println!("{:#?}", map3);

   

}