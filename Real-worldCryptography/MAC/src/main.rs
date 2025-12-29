// 이 파일은 HMAC-SHA256으로 메시지 인증 태그(MAC)를 생성하는 간단한 예제입니다.
// 주석은 러스트의 기본 문법(모듈/크레이트, 제네릭, 슬라이스/벡터, 소유권/빌림 등)을 함께 설명합니다.
// 코드는 변경하지 않고 설명만 덧붙였습니다.

// use: 외부 크레이트(패키지)에서 타입/트레잇을 현재 스코프로 가져옵니다.
// `sha2` 크레이트의 `Sha256` 타입을 사용해 HMAC의 해시 함수로 지정합니다.
use sha2::Sha256;
// `hmac` 크레이트에서 제네릭 HMAC 타입과 `Mac` 트레잇(메서드 집합)을 가져옵니다.
// `Mac` 트레잇은 update/finalize 등의 메서드를 제공합니다.
use hmac::{Hmac, Mac};
// `hex` 크레이트: 바이트를 사람이 읽기 쉬운 16진수 문자열로 인코딩할 때 사용합니다.
//use hex;

// 함수 선언: `send_message`
// - 입력: 비밀키 `key`와 메시지 `message`를 바이트 슬라이스로 받습니다.
//   - `&[u8]`는 "바이트 배열을 빌려 쓰는(참조하는)" 슬라이스 타입입니다.
//   - 소유하지 않으므로 복사 없이 읽기만 합니다(러스트의 빌림/소유권 개념).
// - 반환: `Vec<u8>` (가변 길이 바이트 벡터)로 MAC 태그를 돌려줍니다.
fn send_message(key: &[u8], message: &[u8]) -> Vec<u8> {
    // 비밀키로 HMAC-SHA256 초기화
    // 제네릭 HMAC: `Hmac::<Sha256>`은 해시 함수로 `Sha256`을 사용하는 HMAC 타입을 지정합니다.
    // `new_from_slice(key)`는 키 바이트로 초기화하며, 키 길이가 부적절하면 에러를 반환합니다.
    // `expect("key ok")`는 에러 시 패닉(프로그램 중단)하며 메시지를 표시합니다.
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("key ok");
    // `update`로 메시지 바이트를 MAC 내부 상태에 추가합니다.
    mac.update(message);
    // `finalize()`로 계산을 완료하고 태그를 얻습니다.
    // finalize는 내부 상태를 소비(소유권 이동)하고 결과 타입을 반환합니다.
    // `into_bytes()`로 바이트 배열(태그)을 꺼내고 `to_vec()`으로 `Vec<u8>`로 변환합니다.
    mac.finalize().into_bytes().to_vec() // 인증 태그 반환
}
fn receive_message(key: &[u8], message: &[u8], authentication_tag: &[u8]) -> bool{
    // 기존: let mut mac = Hmac::<Sha256>::new(key);
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("key ok");
    mac.update(message);
    // 기존: mac.Verify(authentication_tag).is_ok()
    mac.verify_slice(authentication_tag).is_ok()
}
fn main() {
    // 바이트 문자열 리터럴: `b"..."`는 UTF-8 문자열이 아닌 "바이트" 리터럴을 생성합니다.
    // 크립토 코드에서는 명확한 바이트 처리가 중요하므로 바이트 리터럴이 자주 쓰입니다.
    let key = b"super-secret-key";
    let msg = b"hello mac";
    // 함수 호출: `&[u8]` 슬라이스를 그대로 전달합니다.
    let tag = send_message(key, msg);
    // 출력: `hex::encode(tag)`로 바이트를 16진수 문자열로 변환해 사람이 읽기 쉽게 표시합니다.
    // `println!`은 매크로로, `{}` 자리에 값을 포맷해 출력합니다.
    println!("HMAC(tag): {}", hex::encode(&tag)); // 16진수로 출력

    let is_ok = receive_message(key, msg, &tag);
    println!("Is the message authentic? {}", is_ok);
}
