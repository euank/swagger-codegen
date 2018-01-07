/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct StoreApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> StoreApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> StoreApiClient<C> {
        StoreApiClient {
            configuration: configuration,
        }
    }
}

pub trait StoreApi {
    fn delete_order(&self, order_id: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_inventory(&self, ) -> Box<Future<Item = ::std::collections::HashMap<String, i32>, Error = Error<serde_json::Value>>>;
    fn get_order_by_id(&self, order_id: i64) -> Box<Future<Item = ::models::Order, Error = Error<serde_json::Value>>>;
    fn place_order(&self, body: ::models::Order) -> Box<Future<Item = ::models::Order, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>StoreApi for StoreApiClient<C> {
    fn delete_order(&self, order_id: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=order_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn get_inventory(&self, ) -> Box<Future<Item = ::std::collections::HashMap<String, i32>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/store/inventory", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::std::collections::HashMap<String, i32>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_order_by_id(&self, order_id: i64) -> Box<Future<Item = ::models::Order, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=order_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::Order, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn place_order(&self, body: ::models::Order) -> Box<Future<Item = ::models::Order, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/store/order", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::Order, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
