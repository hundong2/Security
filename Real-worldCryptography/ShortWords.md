# 보안 축약어 계층 구조 총정리

대표 축약어를 범주별로 묶고, 포함(구성) 관계를 간단히 정리합니다. AEAD, 블록암호 모드, MAC/KDF/PRF/PRP 등을 계층적으로 파악합니다.

## 예시 파일

[NIST SP 800-38D: GCM/GMAC 표준(PDF)](https://csrc.nist.gov/publications/detail/sp/800-38d/final)

## 답변
- 기초 프리미티브(Primitive)
  - PRP: Pseudorandom Permutation(전단사 난수처럼 보이는 함수). 예: AES(블록암호) → “AES는 (강한) PRP로 동작” 가정.
  - PRF: Pseudorandom Function(난수처럼 보이는 키드 함수). 예: HMAC, CMAC, SipHash.
  - Hash: SHA-256, SHA-3(스펀지). MAC 없이 단독 사용 시 인증 없음.
  - MAC: 메시지 인증 코드. 예: HMAC, CMAC, Poly1305, GMAC(GCM의 인증만).
  - KDF: 키 유도 함수. 예: HKDF(HMAC 기반), PBKDF2, scrypt, Argon2.
  - RNG/DRBG: 난수/의사난수(키·IV·Nonce 생성). 예: NIST SP 800-90A DRBG.

- 블록암호 운용 모드(Mode of Operation)
  - CTR: Counter Mode(스트림화). 기밀성만 → 인증 결합 필요.
  - CBC: 이전 블록과 체인. 패딩 필요, 인증 결합 필요(EtM 권장).
  - GCM: CTR(암호화) + GHASH(인증) → AEAD.
  - CCM: CTR + CBC-MAC → AEAD.
  - SIV: Misuse-resistant(오용 내성) AEAD의 핵심 아이디어(예: AES-GCM-SIV).
  - 포함 관계:
    - AES-GCM = AES-CTR + GHASH + Tag(128비트).
    - GMAC = GCM에서 평문=빈 값(인증만).
    - CBC-FKS = AES-CBC + HMAC(Encrypt-then-MAC).

- AEAD 계층(Authenticated Encryption with Associated Data)
  - 개념: 암호화(기밀성) + 인증(무결성/위조 방지) 일체형. AAD(연관 데이터)를 함께 인증.
  - 알고리즘 예:
    - AES-GCM(권장): IV/Nonce(12B 고유), Tag(128b), AAD 포함.
    - ChaCha20-Poly1305: 스트림암호 + MAC 결합 AEAD.
    - AES-GCM-SIV: IV 재사용 실수에 더 강함.
  - 구성 요소:
    - IV/Nonce(유일성 필수), Ciphertext, Tag(상수시간 검증), AAD.

- 인증·검증 관련 개념
  - AAD: 암호문에 포함되지 않지만 태그로 보호되는 헤더/메타데이터.
  - Tag: 인증 결과(예: 128비트). 반드시 상수시간 비교로 검증.
  - EtM: Encrypt-then-MAC(태그 검증 후 복호화). CBC 등에서 필수적 권장.
  - GHASH: GCM/GMAC의 다항식 해시(인증 핵심).

- 프로토콜/표준과 사용 맥락
  - TLS 1.3: HKDF(PRF/KDF) + AEAD(AES-GCM/ChaCha20-Poly1305).
  - JOSE/JWT: A128CBC-HS256(CBC-HMAC AE 구성), A128GCM 등.
  - 저장/전송: AEAD 권장, CBC는 FKS(EtM)로 엄격 운영.

- 주의할 공격/운용 이슈
  - Length Extension Attack: Merkle–Damgård 해시로 만든 MAC(Hash(key||msg)) 취약 → HMAC 사용.
  - Padding Oracle: CBC에서 태그 검증 없이 복호화/에러 누설 시 취약 → EtM로 방지.
  - Nonce 재사용: CTR/GCM에서 기밀성·무결성 붕괴 → Nonce 유일성 보장.

요약 관계도(텍스트)
- AES(블록암호, PRP)
  - 모드: CTR, CBC, GCM, CCM, SIV…
  - GCM = CTR + GHASH → AEAD
    - GMAC = GCM(평문=0) → 인증만
  - CBC-FKS = CBC + HMAC(EtM) → AE 구성
- HMAC: MAC이자 PRF → HKDF(Extract/Expand)의 기반
- AEAD: {암호화 + 인증 + AAD, IV/Nonce, Tag}

### 추가 자료
- [RFC 5116: AEAD 알고리즘 인터페이스](https://www.rfc-editor.org/rfc/rfc5116)
- [RFC 5869: HKDF(HMAC 기반 KDF)](https://www.rfc-editor.org/rfc/rfc5869)
- [NIST SP 800-38A: CBC/CTR 등 운용 모드](https://csrc.nist.gov/publications/detail/sp/800-38a/final)
- [NIST SP 800-38D: AES-GCM/GMAC](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [RFC 8452: AES-GCM-SIV(오용 내성 AEAD)](https://www.rfc-editor.org/rfc/rfc8452)