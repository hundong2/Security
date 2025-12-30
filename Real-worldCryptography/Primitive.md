# 암호학에서 프리미티브(primitive)란?
프로토콜을 만드는 데 쓰이는 최소 단위의 암호 알고리즘/구성요소입니다. 명확한 입력·출력과 보안 목표를 가지며, 서로 조합되어 더 큰 시스템(TLS 등)을 이룹니다.

## 예시 파일
[FIPS 197: Advanced Encryption Standard (AES) — PDF](https://csrc.nist.gov/publications/detail/fips/197/final)

## 답변
핵심 요약:
- 정의: 독립적으로 명세·분석되는 “기본 블록” 암호 도구. 예: 블록암호(AES), 해시(SHA-256), MAC(HMAC/CMAC), PRF/PRP, 스트림암호(ChaCha20), 서명(RSA/ECDSA), 키교환(DH/ECDH), KDF(HKDF).
- 역할: 프리미티브를 안전하게 조합해 프로토콜/시스템(TLS, 메시지 암호화, 저장 데이터 보호)을 구성.
- 보안 속성(예):
  - 블록암호: PRP/강한 PRP 가정, IND-CPA/IND-CCA 모드로 운용
  - 해시: 역상/제2역상/충돌 저항성
  - MAC: 위조불가(UF-CMA)
  - PRF: 무작위 함수와의 구분 불가능성
- 프리미티브 vs 프로토콜:
  - 프리미티브: HMAC, AES, SHA-256 같은 “부품”
  - 프로토콜/구성: AEAD(AES-GCM, ChaCha20-Poly1305), TLS, JWT 서명 등 “부품의 안전한 조합”
- 실무 가이드:
  - 표준화·검증된 프리미티브 사용: AES-GCM/ChaCha20-Poly1305(암호+인증), HMAC-SHA-256(MAC), HKDF(KDF)
  - 도메인 분리와 키 분리 적용(같은 프리미티브를 다른 목적에 재사용하지 않기)
  - 안전한 직렬화와 무작위(Nonce/IV) 관리로 프리미티브의 가정 유지

간단 예(프리미티브 조합의 예시):
- “AES(암호) + HMAC(MAC)” → AEAD 구성(현대에는 AES-GCM/ChaCha20-Poly1305처럼 일체형 AEAD 권장)
- “HMAC(PRF) + HKDF” → 여러 파생 키를 안전하게 유도

### 추가 자료
- [RFC 5869: HKDF—HMAC 기반 키 유도 함수](https://www.rfc-editor.org/rfc/rfc5869)
- [Wikipedia: Cryptographic primitive](https://en.wikipedia.org/wiki/Cryptographic_primitive)