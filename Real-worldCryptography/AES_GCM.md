# AES-GCM AEAD 상세 설명
AES-GCM은 AEAD(연관 데이터 인증 암호화)로, AES-CTR로 기밀성을, GHASH로 무결성/인증을 동시에 제공합니다. 빠르고 병렬화가 쉬우며, 네트워크 프로토콜에서 널리 쓰입니다.

## 예시 파일
[AEAD 예제 코드 (Node.js Web Crypto, hello.js)](https://github.com/hundong2/Security/blob/main/Real-worldCryptography/AEAD/hello.js)

## 답변
- 핵심 개념
  - **AEAD**: 암호화와 인증을 하나의 일체형 구조로 처리. 평문과 함께 추가 메타데이터(AAD)를 인증 가능.
  - **AES-GCM**: 
    - 암호화는 AES-CTR로 수행(키스트림 XOR).
    - 인증은 GHASH(유한체 GF(2^128)에서 다항식 해시)로 태그 생성.
    - 출력은 `(ciphertext, tag)`이며, 태그는 일반적으로 128비트(16바이트).

- 동작 구성
  - **키**: AES-128/192/256 중 선택(실무는 128 또는 256).
  - **IV/Nonce(권장 12바이트=96비트)**: 메시지마다 고유해야 함. 재사용하면 치명적 취약 발생.
  - **AAD(Associated Data)**: 암호문에 포함되지 않지만 인증되는 헤더/메타데이터.
  - **태그(tagLength)**: 보통 128비트 권장(보다 짧게 설정하면 위조 위험↑).

- 내부 원리(요약)
  - **암호화(CTR)**: 같은 키/IV 조합이면 동일 키스트림이 생성됨 → 평문이 XOR로 드러날 수 있으므로 IV 재사용 금지.
  - **인증(GHASH)**: 해시 서브키 `H = E_K(0^128)`로 다항식 누적:
    - 직관적 형태: 
      $$
      \text{GHASH}(H, A, C) = \big( \text{poly}(A) \oplus \text{poly}(C) \oplus \text{length\_block} \big) \cdot H
      $$
      여기서 `poly(·)`는 블록을 GF(2^128) 다항식으로 축적하는 연산. 결과를 CTR 최종 상태와 결합해 인증 태그 생성.

- 보안 포인트(매우 중요)
  - **IV/Nonce 재사용 금지**: 같은 키로 IV를 재사용하면
    - 암호화 측면: 두 평문의 XOR가 노출되어 기밀성 붕괴.
    - 인증 측면: 위조가 가능해져 무결성 붕괴.
  - **IV 선택**:
    - 권장: 12바이트(96비트) IV + 카운터(세션 랜덤 salt + 증가 카운터).
    - 무작위 IV를 쓸 경우, 충돌 확률(생일 경계)을 관리할 만큼 충분히 큰 공간과 모니터링 필요.
  - **태그 길이**: 128비트 사용. 짧은 태그는 위조 확률 증가.
  - **AAD 사용**: 헤더/메타데이터를 AAD로 포함시켜 변조 방지.

- 실무 가이드
  - **모드 선택**: 일반 서비스에는 AES-128-GCM으로 충분. 규정/장기 보존은 AES-256-GCM 고려.
  - **키/IV 관리**: 
    - 키는 KDF(HKDF, Argon2 등)로 유도.
    - IV는 메시지마다 유일 → 세션 랜덤 + 카운터 조합 권장.
  - **태그 검증 실패 처리**: 즉시 실패/폐기(데이터 사용 금지).
  - **오용 내성 필요 시**: **AES-GCM-SIV(RFC 8452)** 같은 misuse-resistant AEAD 고려(IV 재사용 실수에 더 강함).

- 간단 실행 예시(macOS, OpenSSL)
  ```sh
  # 32바이트(256비트) 키와 12바이트 IV 생성
  KEY256=$(openssl rand -hex 32)
  IV=$(openssl rand -hex 12)

  # AES-256-GCM 암호화 (-p로 파라미터 출력)
  openssl enc -aes-256-gcm -K "$KEY256" -iv "$IV" -in plain.txt -out enc.bin -p

  # 복호화
  openssl enc -d -aes-256-gcm -K "$KEY256" -iv "$IV" -in enc.bin -out recovered.txt
  diff plain.txt recovered.txt
  ```

- Node.js(Web Crypto) 개념 확인
  - 파일: hello.js
  - ESM 환경에서 `globalThis.crypto.subtle`를 사용해 `AES-GCM` 키 생성/암복호화.
  - IV는 `crypto.getRandomValues(new Uint8Array(12))`로 생성, `tagLength: 128` 권장.

- AES-CBC-HMAC과 비교
  - **AES-GCM**: 일체형 AEAD, 병렬화 쉬움, 구현 단순(하지만 IV 관리 엄격).
  - **AES-CBC + HMAC**: 암호화/인증 분리(체계적으로 안전하지만 조합 실수 위험—순서·직렬화 주의).

### 추가 자료
- [NIST SP 800-38D: GCM 모드(공식)](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [RFC 5116: AEAD 알고리즘 인터페이스](https://www.rfc-editor.org/rfc/rfc5116)
- [RFC 8452: AES-GCM-SIV(오용 내성)](https://www.rfc-editor.org/rfc/rfc8452)
- [OpenSSL enc 문서(AES-GCM)](https://www.openssl.org/docs/man3.0/man1/openssl-enc.html)
- [Node.js Web Crypto(SubtleCrypto)](https://nodejs.org/api/webcrypto.html#class-subtlecrypto)