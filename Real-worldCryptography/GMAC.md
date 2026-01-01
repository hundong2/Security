# GMAC란? 

AES-GCM에서 “암호화 없이 인증만” 수행하는 모드입니다. 평문을 비워 둔 GCM으로 AAD(연관 데이터)만 GHASH로 인증 태그를 만들며, 태그는 보통 128비트입니다.

## 예시 파일

[OpenSSL: EVP_MAC-GMAC 문서](https://www.openssl.org/docs/man3.0/man7/EVP_MAC-GMAC.html)

## 답변

- 정의: GMAC = Galois Message Authentication Code. AES-GCM의 인증 부분만 사용해 메시지(또는 헤더)를 MAC 처리합니다. 평문 길이가 0인 GCM과 동일합니다.
- 입력/파라미터:
  - 키: AES-128/192/256 중 선택.
  - Nonce/IV: 12바이트 권장. 같은 키로 재사용하면 위조가 가능해집니다.
  - AAD: 인증할 바이트열(헤더/메타데이터 등). GMAC에서는 이것이 “메시지”입니다.
  - 태그 길이: 128비트 권장(짧게 하면 위조 확률 증가).
- 내부 동작(요약):
  - 해시 서브키 H = AES_K(0^128)을 만들고, AAD를 GF(2^128) 다항식(GHASH)로 누적.
  - CTR 초기값(J0)을 암호화한 값과 GHASH 결과를 XOR해 최종 태그 생성.
- 특징/비교:
  - 장점: 빠르고 병렬화 쉬움(하드웨어 친화적). 인증만 필요할 때 적합.
  - 주의: HMAC/CMAC과 달리 “nonce(IV) 유일성”이 안전의 핵심입니다. IV 재사용은 치명적입니다.
  - 용도: 패킷/헤더 인증, 저장소 메타데이터 검증 등 “암호화 없이 변조 방지”가 필요한 경우.

간단 예시(Node.js Web Crypto: AAD만 인증해 태그 생성)
````javascript
// GMAC을 AES-GCM으로 구현: 평문은 빈 값, AAD만 인증
const crypto = globalThis.crypto; // Node v19+ (ESM)
const key = await crypto.subtle.generateKey({ name: 'AES-GCM', length: 128 }, false, ['encrypt','decrypt']);
const iv = crypto.getRandomValues(new Uint8Array(12));            // 12바이트 IV(유일해야 함)
const aad = new TextEncoder().encode('header-to-auth');            // 인증할 데이터
const params = { name: 'AES-GCM', iv, additionalData: aad, tagLength: 128 };

// 빈 평문 → 반환 버퍼는 사실상 태그만 포함
const buf = await crypto.subtle.encrypt(params, key, new Uint8Array());
console.log('GMAC tag (hex):', Buffer.from(new Uint8Array(buf)).toString('hex'));
````

### 추가 자료

- [NIST SP 800-38D: GCM/GMAC 표준](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [RFC 5116: AEAD 알고리즘 인터페이스](https://www.rfc-editor.org/rfc/rfc5116)
- [AES-GCM-SIV(오용 내성) — RFC 8452](https://www.rfc-editor.org/rfc/rfc8452)