# AES에서의 CBC 모드란?
AES 블록암호를 “이전 암호문 블록과 엮어” 암호화하는 운용 모드입니다. IV로 시작해 각 평문 블록을 직전 암호문과 XOR 후 AES로 암호화합니다.

## 예시 파일
[NIST SP 800-38A (블록암호 운용 모드 표준, CBC 포함)](https://csrc.nist.gov/publications/detail/sp/800-38a/final)

## 답변
- 정의: CBC(Cipher Block Chaining) 모드는 블록 단위로 다음 식을 사용합니다.  
  C₀ = IV, Cᵢ = Eₖ(Pᵢ ⊕ Cᵢ₋₁), Pᵢ = Dₖ(Cᵢ) ⊕ Cᵢ₋₁. AES는 블록 크기 128비트(16바이트)입니다.
- IV(초기 벡터): 16바이트(블록 크기와 동일). 암호화할 때마다 “고유/예측불가” IV가 필요합니다. 같은 키로 IV 재사용은 기밀성 약화로 이어집니다.
- 패딩: 평문 길이가 16바이트 배수가 아니면 PKCS#7 패딩을 붙여 블록 정렬합니다.
- 보안 성질: 기밀성만 제공합니다. CBC는 “가변 가능한(malleable)” 구조라 태그 검증 없이 쓰면 변조를 탐지하지 못합니다. 패딩 오라클 취약과 같은 구현 실수에도 민감하므로, 반드시 인증(MAC 또는 AEAD)과 함께 사용해야 합니다.
- 권장 구성: Encrypt-then-MAC(예: AES-CBC + HMAC, 일명 CBC-FKS) 또는 일체형 AEAD(AES-GCM/ChaCha20-Poly1305)를 사용하세요.
- 성능 특성: 암호화는 직전 블록 의존으로 병렬화가 어렵습니다. 복호화는 블록들을 갖고 있으면 병렬 처리가 가능하지만 구현 복잡도와 환경에 따라 달라집니다.

간단 실습(OpenSSL, zsh):
```sh
# 32바이트(256비트) 키와 16바이트 IV 생성
KEY=$(openssl rand -hex 32)
IV=$(openssl rand -hex 16)

# AES-256-CBC 암호화(PKCS#7 패딩 기본)
openssl enc -aes-256-cbc -K "$KEY" -iv "$IV" -in plain.txt -out cipher.bin -p

# 복호화
openssl enc -d -aes-256-cbc -K "$KEY" -iv "$IV" -in cipher.bin -out recovered.txt
diff plain.txt recovered.txt
```
주의: 위 구성은 “인증 없는” 암호화입니다. 실제 서비스에서는 `IV || ciphertext`에 대해 HMAC 태그를 생성·검증하거나, AEAD(AES-GCM 등)를 사용해야 합니다.

### 추가 자료
- [AES-CBC-HMAC(A128CBC-HS256) — RFC 7518 JWA](https://www.rfc-editor.org/rfc/rfc7518#section-5.2.3)
- [OpenSSL enc 매뉴얼(AES-CBC)](https://www.openssl.org/docs/man3.0/man1/openssl-enc.html)