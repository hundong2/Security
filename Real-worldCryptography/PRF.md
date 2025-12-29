# 의사 난수 함수(PRF)란?
키가 있는 함수 F_k(x)가 외부 관찰자에게 “무작위 함수”처럼 보이도록 설계된 함수입니다. 공격자는 F_k를 오라클로 호출해도 진짜 난수 함수와 구분하기 어려워야 합니다.
## 예시 파일
[RFC 5869: HKDF—HMAC 기반 키 유도 함수](https://www.rfc-editor.org/rfc/rfc5869)
## 답변
핵심 개념:
- 정의: PRF는 키 k를 가진 결정적 함수 F_k(x)로, 임의의 입력 x에 대해 출력이 난수처럼 보입니다(구분 불가능성: indistinguishability).
- 왜 중요한가: 안전한 KDF·프로토콜에서 “새 키·Nonce·패킷 키”를 유도할 때, 예측 불가능한 바이트열이 필요합니다.
- 흔한 실현 방식:
  - HMAC 기반: F_k(x) = HMAC-SHA256(k, x). HMAC은 표준적으로 PRF로 사용됩니다(TLS, HKDF).
  - 블록암호 기반: AES-CTR/AES-CMAC 등도 PRF로 사용 가능.
- PRF vs 해시/MAC:
  - 해시는 키가 없어 PRF가 아닙니다.
  - HMAC은 “인증(MAC)”에도 쓰이지만, 입력에 따라 난수처럼 보이는 성질로 PRF로도 쓰입니다.
- 사용처:
  - HKDF(Extract/Expand): HMAC을 PRF로 사용해 여러 키를 도출.
  - TLS: TLS 1.3은 HKDF, 1.2는 HMAC 기반 PRF로 비밀 자료를 확장.
- 베스트 프랙티스:
  - 검증된 빌딩블록(HMAC-SHA256)을 PRF로 사용.
  - 도메인 분리(info/label)로 목적별 출력 구분.
  - 충분한 출력 길이(≥32바이트)와 고정 인코딩 적용.

간단 예시(파이썬: HMAC-SHA256 PRF)
````python
import hmac, hashlib

def prf(key: bytes, x: bytes) -> bytes:
    return hmac.new(key, x, hashlib.sha256).digest()

k = b"secret-key"
out1 = prf(k, b"context-A")
out2 = prf(k, b"context-B")  # 서로 다른 컨텍스트로 도메인 분리
print(len(out1), out1.hex())
print(len(out2), out2.hex())
````

### 추가 자료
- [RFC 5246: TLS 1.2 PRF 정의](https://www.rfc-editor.org/rfc/rfc5246)
- [RFC 8446: TLS 1.3(HKDF 기반 키 스케줄)](https://www.rfc-editor.org/rfc/rfc8446)
- [NIST SP 800-56C: KDF(Extract/Expand) 일반 프레임워크](https://csrc.nist.gov/publications/detail/sp/800-56c/rev-2/final)