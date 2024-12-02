use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;
use std::{iter::zip, net::{Ipv4Addr, Ipv6Addr}};

#[derive(Deserialize)]
pub struct DecryptQuery {
    pub from: Ipv4Addr,
    pub key: Ipv4Addr,
}

pub async fn decrypt_destination(
    Query(params): Query<DecryptQuery> 
) -> impl IntoResponse {
    let from_vec = params.from.octets();
    let key_vec = params.key.octets();
    
    let sum_vec: Vec<u8> = zip(from_vec, key_vec).map(|(a, b)| a.wrapping_add(b)).collect();

    let destination = Ipv4Addr::new(sum_vec[0], sum_vec[1], sum_vec[2], sum_vec[3]);

    destination.to_string()
}

#[derive(Deserialize)]
pub struct ReverseDecryptQuery {
    pub from: Ipv4Addr,
    pub to: Ipv4Addr
}

pub async fn decrypt_key(
    Query(params): Query<ReverseDecryptQuery>
) -> impl IntoResponse {
    let from_vec = params.from.octets();
    let to_vec = params.to.octets();

    let sub_vec: Vec<u8> = zip(from_vec, to_vec).map(|(a, b)| b.wrapping_sub(a)).collect();

    let key = Ipv4Addr::new(sub_vec[0], sub_vec[1], sub_vec[2], sub_vec[3]);

    key.to_string()
}

#[derive(Deserialize)]
pub struct DecryptV6Query {
    pub from: Ipv6Addr,
    pub key: Ipv6Addr,
}

pub async fn decrypt_destination_v6(
    Query(params): Query<DecryptV6Query>
) -> impl IntoResponse {
    let from_vec = params.from.segments();
    let key_vec = params.key.segments();
    
    let sum_vec: Vec<u16> = zip(from_vec, key_vec).map(|(a, b)| a ^ b).collect();

    let destination = Ipv6Addr::new(
        sum_vec[0],
        sum_vec[1],
        sum_vec[2],
        sum_vec[3],
        sum_vec[4],
        sum_vec[5],
        sum_vec[6],
        sum_vec[7]
    );

    destination.to_string()
}

#[derive(Deserialize)]
pub struct ReverseDecryptV6Query {
    pub from: Ipv6Addr,
    pub to: Ipv6Addr
}


pub async fn decrypt_key_v6(
    Query(params): Query<ReverseDecryptV6Query>
) -> impl IntoResponse {
    let from_vec = params.from.segments();
    let to_vec = params.to.segments();

    let sub_vec: Vec<u16> = zip(from_vec, to_vec).map(|(a, b)| b ^ a).collect();

    let key = Ipv6Addr::new(
        sub_vec[0],
        sub_vec[1],
        sub_vec[2],
        sub_vec[3],
        sub_vec[4],
        sub_vec[5],
        sub_vec[6],
        sub_vec[7]
    );

    key.to_string()
}