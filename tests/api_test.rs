extern crate jsonapi;
extern crate serde_json;

use jsonapi::api::*;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_json_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    };

    s
}

#[test]
fn it_works() {
    let resource = Resource {
        _type: format!("test"),
        id: format!("123"),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
    };

    assert_eq!(resource.id, format!("123"));

    let serialized = serde_json::to_string(&resource).unwrap();
    let deserialized: Resource = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.id, resource.id);

    let document = Document {
        data: None,
        errors: None,
        meta: None,
    };

    assert_eq!(document.has_data(), false);
    assert_eq!(document.has_errors(), false);
    assert_eq!(document.has_meta(), false);

    assert_eq!(document.is_valid(), false);

}

#[test]
fn document_can_be_valid() {
    let resource = Resource {
        _type: format!("test"),
        id: format!("123"),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
    };

    let errors = JsonApiErrors::new();

    let invalid_document = Document {
        data: None,
        errors: None,
        meta: None,
    };

    assert_eq!(invalid_document.is_valid(), false);

    let document_with_data = Document {
        data: Some(PrimaryData::Single(resource)),
        errors: None,
        meta: None,
    };

    assert_eq!(document_with_data.is_valid(), true);

    let document_with_errors = Document {
        data: None,
        errors: Some(errors),
        meta: None,
    };

    assert_eq!(document_with_errors.is_valid(), true);

}

#[test]
fn error_from_json_string() {

    let serialized = r#"
        {"id":"1", "links" : {}, "status" : "unknown", "code" : "code1", "title" : "error-title", "detail": "error-detail"}
        "#;
    let error: Result<JsonApiError, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(error.is_ok(), true);
    assert_eq!(error.unwrap().id, "1");
}

#[test]
fn single_resource_from_json_string() {
    let serialized =
        r#"{ "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }"#;
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn multiple_resource_from_json_string() {
    let serialized = r#"[
            { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
            { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
            { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }
        ]"#;
    let data: Result<Resources, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn no_data_response_from_json_string() {
    let serialized = r#"{
            "data" : null
        }"#;
    let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn single_data_response_from_json_string() {
    let serialized = r#"{
            "data" : {
                "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {}
            }
        }"#;
    let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn multiple_data_response_from_json_string() {
    let serialized = r#"{
            "data" : [
                { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
                { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
                { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }
            ]
        }"#;
    let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn api_response_from_json_file() {

    let s = ::read_json_file("data/results.json");
    let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&s);

    match data {
        Ok(res) => {
            match res.data {
                PrimaryData::Multiple(arr) => {
                    assert_eq!(arr.len(), 1);
                }
                PrimaryData::Single(_) => {
                    println!("api_response_from_json_file : Expected one Resource in a vector, \
                              not a direct Resource");
                    assert!(false);
                }
                PrimaryData::None => {
                    println!("api_response_from_json_file : Expected one Resource in a vector");
                    assert!(false);
                }
            }
        }
        Err(err) => {
            println!("api_response_from_json_file : Error: {:?}", err);
            assert!(false);
        }
    }
}

#[test]
fn api_response_collection_from_json_file() {

    let s = ::read_json_file("data/collection.json");
    let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&s);

    match data {
        Ok(res) => {

            match res.data {
                PrimaryData::Multiple(arr) => {
                    assert_eq!(arr.len(), 1);
                }
                PrimaryData::Single(_) => {
                    println!("api_response_collection_from_json_file : Expected one Resource in \
                              a vector, not a direct Resource");
                    assert!(false);
                }
                PrimaryData::None => {
                    println!("api_response_collection_from_json_file : Expected one Resource in \
                              a vector");
                    assert!(false);
                }
            }

            match res.included {
                Some(arr) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0].id, "9");
                    assert_eq!(arr[1].id, "5");
                    assert_eq!(arr[2].id, "12");
                }
                None => {
                    println!("api_response_collection_from_json_file : Expected three Resources \
                              in 'included' in a vector");
                    assert!(false);
                }
            }

            match res.links {
                Some(links) => {
                    assert_eq!(links.len(), 3);
                }
                None => {
                    println!("api_response_collection_from_json_file : expected links");
                    assert!(false);
                }
            }

        }
        Err(err) => {
            println!("api_response_collection_from_json_file : Error: {:?}", err);
            assert!(false);
        }
    }
}
