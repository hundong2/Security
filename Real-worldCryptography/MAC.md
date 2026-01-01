# Message Authentication Code(MAC)란?

비밀 키로 메시지에 짧은 인증 태그(tag)를 붙여, 수신자가 “메시지가 변조되지 않았고, 키를 가진 발신자가 만든 것”임을 확인하게 해주는 기술입니다.

## 예시 파일

[RustCrypto HMAC README(예제 코드)](https://raw.githubusercontent.com/RustCrypto/MACs/master/hmac/README.md)

## 답변

핵심 요약:
- 목적: 무결성·출처 인증. 수신자는 동일한 키로 태그를 재계산해 일치 여부를 확인합니다.
- 구성: 입력으로 key와 message, 출력으로 인증 태그(tag). 태그 길이는 보통 128~256비트.
- 보안 목표(UF-CMA): 키를 모르는 공격자가 이전 쿼리(메시지·태그)를 봐도 “새 메시지”의 올바른 태그를 만들 수 없어야 합니다.
- 대표 알고리즘:
  - HMAC-SHA-256/512: 가장 널리 쓰이는 표준 MAC(또는 PRF로도 사용).
  - CMAC(AES 기반), Poly1305(매우 빠른 MAC), GMAC(GCM의 인증만).
- 주의점(설계/운용):
  - 해시만으로 MAC을 만들지 말 것(Hash(key||msg) 등은 길이 확장 공격에 취약). HMAC/CMAC 사용.
  - 태그 비교는 상수시간 비교로 수행(값에 따라 일찍 종료하지 않기).
  - 직렬화는 경계가 명확해야 함(길이 접두, AAD 포함 등). 모호한 연결은 의미 위조로 이어짐.
  - AEAD와의 관계: MAC은 “인증만”, AEAD(AES-GCM 등)는 “암호화+인증 일체형”. 전송 데이터 보호에는 AEAD 권장, 인증만 필요하면 MAC.

간단 비교:
- MAC vs 서명: MAC은 대칭키(양쪽 같은 키), 법적 비부인성 없음. 서명은 비대칭키(개인키/공개키), 비부인성 제공.
- MAC vs 해시: 해시는 키 없음→출처 인증 불가. MAC은 키 포함→출처·무결성 보장.

### 추가 자료

- [RFC 2104: HMAC 표준](https://www.rfc-editor.org/rfc/rfc2104)
- [NIST SP 800-38B: CMAC(AES 기반 MAC)](https://csrc.nist.gov/publications/detail/sp/800-38b/final)
- [RFC 5116: AEAD 알고리즘 인터페이스](https://www.rfc-editor.org/rfc/rfc5116)
- [Rust hmac 크레이트 문서](https://docs.rs/hmac/latest/hmac/)
- [Go crypto/subtle(상수시간 비교)](https://pkg.go.dev/crypto/subtle#ConstantTimeCompare)