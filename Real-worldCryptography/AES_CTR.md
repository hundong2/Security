# AES-CTR에서 CTR은 무엇인가?
CTR은 “Counter Mode(카운터 모드)”로, 블록암호(AES)를 스트림암호처럼 사용하게 해주는 운용 모드입니다. Nonce(IV)와 증가하는 카운터 블록을 AES로 암호화해 키스트림을 만들고, 이를 평문과 XOR하여 암호문을 얻습니다.

## 예시 파일
[NIST SP 800-38A: 블록암호 운용 모드 (PDF)](https://csrc.nist.gov/publications/detail/sp/800-38a/final)

## 답변
- 핵심 개념:
  - 키스트림 생성: `S_i = E_K(N || ctr_i)`를 계산해 바이트 스트림을 얻고, `C_i = P_i ⊕ S_i`로 암호화합니다. 복호도 동일하게 `P_i = C_i ⊕ S_i`.
  - Nonce/IV: 키당 메시지마다 “유일”해야 합니다. 같은 키로 같은 Nonce를 재사용하면 두 평문의 XOR가 노출됩니다(기밀성 붕괴).
  - 특징: 패딩이 필요 없고(바이트 단위 처리), 병렬화가 쉬우며, 랜덤 접근이 편합니다.
  - 주의: CTR 자체는 “기밀성”만 제공합니다. 반드시 인증을 결합해야 합니다(예: AES-GCM, 또는 AES-CTR + HMAC의 Encrypt-then-MAC).

- 실무 베스트 프랙티스:
  - Nonce(IV) 크기: AES-CTR 입력 블록은 128비트이므로 보통 `nonce || counter`로 16바이트를 구성합니다(예: 12B nonce + 4B 카운터).
  - Nonce 관리: 세션별 랜덤 nonce + 증가 카운터 방식 권장. 절대 재사용 금지.
  - 인증: AEAD(AES-GCM/ChaCha20-Poly1305) 사용 또는 HMAC과 결합해 태그 검증을 먼저 수행.

- OpenSSL 예시(macOS, zsh):
  ```sh
  # 16바이트(128비트) 키와 16바이트 IV(Nonce||Counter) 생성
  KEY=$(openssl rand -hex 16)
  IV=$(openssl rand -hex 16)

  # AES-128-CTR로 암호화(패딩 없음)
  openssl enc -aes-128-ctr -K "$KEY" -iv "$IV" -in plain.txt -out cipher.bin -p

  # 복호화
  openssl enc -d -aes-128-ctr -K "$KEY" -iv "$IV" -in cipher.bin -out recovered.txt
  diff plain.txt recovered.txt
  ```
  참고: 위 구성은 인증이 없습니다. 실제 서비스에서는 AES-GCM 같은 AEAD를 권장합니다.

### 추가 자료
- [FIPS 197: AES 표준](https://csrc.nist.gov/publications/detail/fips/197/final)
- [OpenSSL enc 문서(AES-CTR)](https://www.openssl.org/docs/manmaster/man1/openssl-enc.html)