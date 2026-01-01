# 논스(Nonce)란?

논스(Nonce)는 **“한 번만 써야 하는 값(또는 한 번만 쓰이도록 만들어야 하는 값)”**으로, 암호 프로토콜에서 **재사용을 막고(리플레이/키스트림 재사용 방지)** 안전하게 암호화·인증을 수행하기 위해 메시지마다 넣는 입력값입니다.

## 예시 파일

[Node.js WebCrypto AES-GCM 테스트 예시 파일](https://github.com/nodejs/node/blob/v22.x/test/parallel/test-webcrypto-aes-gcm.js)

## 답변

### 1) Nonce가 왜 필요하나?

같은 키로 암호화를 여러 번 하면, 매번 **다른 “상태/시작점”**이 필요합니다. Nonce는 그 역할을 해서 다음을 보장합니다.

- **CTR/GCM 같은 스트림 계열**: 같은 키로 같은 nonce를 쓰면 **같은 키스트림이 재사용**될 수 있어 기밀성이 깨집니다.
- **AEAD(AES-GCM, ChaCha20-Poly1305)**: nonce가 재사용되면 보통 **기밀성뿐 아니라 무결성(위조 방지)도 크게 약화**됩니다.

요약: **nonce 재사용은 “치명적 실수”**로 취급됩니다.

---

### 2) Nonce vs IV (헷갈리는 부분 정리)

문서/라이브러리에서 `IV`라는 용어를 많이 쓰는데, 모드마다 의미가 조금 다릅니다.

- **CBC의 IV**
  - 목적: 첫 블록을 랜덤화(동일 평문 패턴 제거)
  - 요구: 일반적으로 **예측 불가능(랜덤)** 이길 권장 + (실무적으로) **재사용 금지**
- **CTR/GCM/ChaCha20-Poly1305의 IV/Nonce**
  - 목적: 카운터/키스트림의 시작점을 바꿈
  - 요구: 핵심은 **“키당 유일(unique)”** (예측 불가능까지 항상 필수는 아니지만, 랜덤으로 만드는 경우가 많음)

즉, **CTR/GCM에서는 IV를 “nonce”라고 부르는 경우가 많고**, 핵심 요구사항은 **유일성**입니다.

---

### 3) 재사용하면 뭐가 망가지나? (초보자용 직관)

CTR처럼 “키스트림 XOR” 구조를 생각하면 됩니다.

- 암호화: `C = P XOR KS(nonce, key)`
- 같은 key+nonce를 재사용하면 KS가 같아짐  
  → 두 암호문을 XOR하면  
  `C1 XOR C2 = (P1 XOR KS) XOR (P2 XOR KS) = P1 XOR P2`  
  즉, 공격자가 **평문끼리의 관계를 바로 얻습니다.** (기밀성 붕괴)

AES-GCM/ChaCha20-Poly1305는 여기에 “인증(tag)”까지 붙는데, nonce 재사용은 **인증 안전성까지 흔들 수 있어** 더 위험합니다.

---

### 4) nonce는 랜덤이어야 하나? 카운터여야 하나?

둘 다 가능합니다. 중요한 건 “키당 유일”하게 관리하는 것입니다.

- **랜덤 nonce(예: 96-bit 랜덤)**  
  - 구현이 쉬움  
  - 단점: 아주 많은 메시지를 암호화하면(“생일 문제”) **충돌 확률 관리**가 필요
- **카운터 nonce(증가 값)**  
  - 충돌 방지에 유리(관리만 잘하면 0)  
  - 단점: 상태 저장/동기화가 필요할 수 있음
- 실무에서 흔한 패턴: **세션별 랜덤 prefix + 메시지 카운터**(유일성 확보 + 관리 용이)

---

### 5) 자주 보는 길이(대표값)

- **AES-GCM**: nonce(IV) **12바이트(96비트)** 권장
- **ChaCha20-Poly1305(IETF)**: nonce **12바이트(96비트)**
- **AES-CBC**: IV **16바이트(블록 크기와 동일)**

---

### 6) 결론(한 줄 규칙)

- **AEAD/CTR 계열에서는 “같은 키로 nonce(IV) 재사용 금지”가 1순위 규칙**입니다.
- nonce는 보통 **전송 데이터에 포함**되어도 괜찮습니다(비밀이 아니라 “유일성”이 핵심). 단, 프로토콜이 요구하는 경우는 예외가 있을 수 있습니다.

### 추가 자료

- [NIST SP 800-38D: GCM/GMAC (Nonce/IV 요구사항)](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [RFC 8439: ChaCha20-Poly1305 (Nonce 규칙)](https://www.rfc-editor.org/rfc/rfc8439)
- [RFC 5116: AEAD 개념 정리](https://www.rfc-editor.org/rfc/rfc5116)

# 암호화·복호화에 같은 Nonce가 필요한 이유

**복호화는 암호화 때 쓴 것과 “같은 nonce(IV)”가 필요**합니다. 다만 중요한 규칙은 **“같은 키로는 nonce를 재사용하면 안 된다”**이지, “nonce를 숨겨야 한다”는 뜻은 아닙니다.

## 예시 파일

[Node.js WebCrypto AES-GCM 테스트 예시 파일](https://github.com/nodejs/node/blob/v22.x/test/parallel/test-webcrypto-aes-gcm.js)

## 답변

### 1) 왜 복호화에 같은 nonce가 필요하나? (CTR/GCM 직관)

AES-CTR/GCM/ChaCha20 같은 방식은 내부적으로 **키스트림**을 만들어 평문과 XOR합니다.

- 암호화: `C = P ⊕ KS(key, nonce)`
- 복호화: `P = C ⊕ KS(key, nonce)`

즉 **같은 KS를 다시 만들어야** 원래 평문이 나오므로, 복호화 측은 **같은 (key, nonce)** 조합을 알아야 합니다.  
그래서 nonce는 보통 **암호문과 함께 전송/저장**합니다(비밀이 아니라 “식별자/시작값”에 가깝습니다).

### 2) “같은 nonce 필요”와 “nonce 재사용 금지”는 모순이 아님

- **한 메시지**를 복호화하려면 그 메시지에 사용된 nonce가 필요 → 그래서 같이 보냄.
- 하지만 **다른 메시지**를 같은 키로 암호화할 때 nonce를 또 쓰면 안 됨 → **키당 유일성**이 규칙.

정리:

- 메시지 1: (key K, nonce N1)로 암호화 → 복호화도 (K, N1)
- 메시지 2: (key K, nonce N2)로 암호화 → 복호화도 (K, N2)
- 금지: (key K, nonce N1)을 메시지 1과 2에서 둘 다 사용

### 3) AES-GCM에서는 nonce가 태그 검증에도 필요

GCM은 암호문 뿐 아니라 **태그(tag)** 도 검증해야 합니다. 태그 계산에도 nonce가 들어가므로, nonce가 틀리면:

- 복호화 결과가 의미 없는 값이 될 뿐 아니라
- **태그 검증도 실패**합니다(정상 동작)

### 4) nonce는 공개해도 되나?

대부분의 AEAD/CTR 계열에서 nonce는 **공개 값**으로 취급합니다(암호문 옆에 붙음).  
단, 요구사항은 반드시 지켜야 합니다:

- **유일성**(같은 키로 재사용 금지)
- 구현에 따라 **예측 가능성**까지 요구하는 경우도 있음(프로토콜/표준에 따름)

### 추가 자료

- [NIST SP 800-38D: GCM/GMAC (Nonce/IV 요구사항)](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [RFC 8439: ChaCha20-Poly1305 (Nonce 규칙)](https://www.rfc-editor.org/rfc/rfc8439)
- [RFC 5116: AEAD 개념 정리](https://www.rfc-editor.org/rfc/rfc5116)