# SipHash란?
짧은 입력을 빠르게 처리하기 위한 64비트 키드 해시(키드 PRF/MAC)로, 해시 테이블 해시-홍수(DoS) 방어에 널리 쓰입니다. HMAC 같은 강한 MAC 대체가 아니라 “짧은 메시지용 PRF”입니다.
## 예시 파일
[SipHash: a fast short-input PRF (원 논문 PDF)](https://131002.net/siphash/siphash.pdf)
## 답변
핵심 요약:
- 정체성: SipHash는 키 k를 가진 결정적 함수 F_k(x)로, 출력이 난수처럼 보이게 설계된 “짧은 입력용 PRF/MAC”입니다. 대표 변형은 SipHash-2-4(라운드 2/4)와 SipHash-1-3입니다.
- 목적: 해시 테이블의 버킷 선택에 키드 해시를 사용해 공격자가 충돌을 의도적으로 유도하는 “해시-홍수(많은 키를 같은 버킷으로)”를 막습니다. Python, Ruby, Rust의 HashMap 기본 해시로 채택되었습니다.
- 출력/보안: 64비트 태그(half 변형은 32비트). 짧은 메시지와 테이블 해시에 적합하지만, 장문 데이터의 강한 인증(HMAC/Poly1305 수준)을 요구하는 용도에는 부족할 수 있습니다.
- 변형과 트레이드오프:
  - SipHash-2-4: 안전성 우선(기본 권장).
  - SipHash-1-3: 성능을 위해 라운드를 줄인 변형(테이블 해시에서 자주 사용).
  - HalfSipHash: 32비트 출력(공간·속도 최적화, 보안 비트 축소).
- 실무 사용 지침:
  - “키드 해시(비밀 키 포함)”로만 의미가 있습니다. 키 없이 쓰면 일반 해시와 동일하게 공격에 취약합니다.
  - 해시 테이블/짧은 메시지 인증에는 적합. 파일 무결성/네트워크 프레임 장기 보호에는 HMAC-SHA256, Poly1305 등을 권장.
  - 도메인 분리(태그/라벨)를 넣어 목적 혼용을 방지하세요.

Rust 예제 코드(siphasher 크레이트로 SipHash-2-4/1-3 계산):
````rust
use siphasher::sip::{SipHasher, SipHasher13};
use std::hash::Hasher;

fn main() {
    // 128비트 키(두 u64). 실무에서는 랜덤 키를 사용하세요.
    let k0 = 0x0706050403020100u64;
    let k1 = 0x0f0e0d0c0b0a0908u64;

    // SipHash-2-4
    let mut h24 = SipHasher::new_with_keys(k0, k1);
    h24.write(b"AliceBob10015");        // 짧은 메시지
    let tag64_24 = h24.finish();        // 64비트 태그
    println!("SipHash-2-4: {:016x}", tag64_24);

    // SipHash-1-3
    let mut h13 = SipHasher13::new_with_keys(k0, k1);
    h13.write(b"AliceBob10015");
    let tag64_13 = h13.finish();
    println!("SipHash-1-3: {:016x}", tag64_13);
}
````

최근 동향(요약):
- 표준/알고리즘 변동 없음: SipHash-2-4 권장 기조 유지.
- 채택 확대/유지: 파이썬(Python 3.4+는 str/bytes 해시에 siphash24), 루비, Rust 표준 라이브러리 HashMap의 기본 해셔는 여전히 SipHash 기반(RandomState). 더 빠른 대안(AHash 등)은 외부 크레이트로 선택 사용.
- 보안 포인트: 테이블 해시 목적에는 충분하나, 데이터 인증에는 64비트 태그가 짧으므로 강한 MAC이 필요하면 HMAC/Poly1305를 사용.

비교 요약:
- SipHash: 키드 PRF/MAC, 짧은 입력·테이블 해시용, 64비트 출력.
- HMAC-SHA256: 강한 MAC/PRF, 긴 입력·데이터 무결성에 적합, 256비트 출력.
- Poly1305: 아주 빠른 MAC(128비트 태그), “동일 키를 여러 메시지에 안전하게” 쓰도록 설계됨(ChaCha20-Poly1305 등).

### 추가 자료
- [SipHash 위키백과](https://en.wikipedia.org/wiki/SipHash)
- [Rust siphasher 크레이트 문서](https://docs.rs/siphasher/latest/siphasher/)
- [PEP 456: 파이썬에서 SipHash 도입](https://peps.python.org/pep-0456/)
- [Rust HashMap RandomState 문서](https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html)