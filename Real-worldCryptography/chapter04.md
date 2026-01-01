# Authentication 

- 인증 암호학 

## AES 의 인터페이스

- 알고리즘은 가변 길이 키를 사용
- 128비트의 평문을 사용
- 128비트의 암호문을 사용 

- 고정 된 크기의 평문을 암호화 하기 때문에 `block cipher(블록암호)`라고 한다.  
- [PSP](./pseudorandom_permutations.md). 
- [AES round function](./AES_Round.md). 

## padding 

- 평문의블록이 128비트 (16바이트) 보다 작은 경우 패딩 바이트를 선택
- 가장 널리 쓰이는 패딩 메커니즘은 `PKCS#7 패딩`  

# AES-CBC-HMAC

### Diagram: Encrypt-then-MAC (FKS)

```mermaid
flowchart LR
  P[Plaintext] --> Pad[PKCS#7 Padding]
  Pad -->|AES-CBC (K_enc, IV)| C[Ciphertext]

  AAD[AAD] -- concat --> H[HMAC-SHA-256 (K_mac)]
  IV[IV (16B)] -- concat --> H
  C -- concat --> H
  H --> T[Tag]

  subgraph Output
    AAD
    IV
    C
    T
  end
```

### Diagram: Verify-then-Decrypt

```mermaid
flowchart LR
  AAD --> H2[HMAC-SHA-256 (K_mac)]
  IV --> H2
  C[Ciphertext] --> H2
  H2 --> Compare{Constant-time compare}
  T[Tag] --> Compare
  Compare -- OK --> Dec[AES-CBC Decrypt (K_enc, IV)] --> Unpad[Remove PKCS#7] --> P2[Plaintext]
  Compare -- FAIL --> Drop[Reject]
```

> Notes
> - IV(16B)는 키당 메시지마다 고유해야 하며 재사용 금지.
> - 태그 검증을 반드시 복호화 전에 수행(패딩 오라클 방지).
> - 태그 비교는 상수시간 비교로 처리.

# 올인원 구조 인증 

## AEAD 

- 연관 데이터 인증 암호화 ( Authenticated encryption with associated data): 일체형 구조 
- [Example code ](./AEAD/hello.js). 

## AES-GCM AEAD 

- 갈루아/카운터 모드(Galois/Counter Mode)가 있는 AES
- AES 에 대한 하드웨어 자원을 활용하여, 고성능을 위해 이를 효율적으로 구현할 수 있는 MAC(GMAC)을 사용하여 설계됨. 
- [AES-GCM](./AES_GCM.md). 

## AES CBC

### AES CBC HMAC

- CBC에서 공격자는 여전히 암호문과 IV를 수정할 수 있다.  실제로 이를 방지하는 무결성 메커니즘은 없다.  
- 일반적인 구조 

### ChaCha20-Poly1305

- ChaCha20 stream 암호와 Poly1350 MAC이라는 두가지 알고리즘의 조합이다. 
- 하드웨어 자원을 사용할 수 없을때의 AES와 달리 소프ㅡ웨어에서 사용할 때 빠른 속도를 위해 설계 됨. 

# 용어 

- `CBC` : ciper block chaining. 
- `IV` :  Initialize vector. 
  - CBC작동 모드에서 고유해야(반복할 수 없음), 예측할 수 없는 값. 
- `ECB` : electronic codebook.  
  - 전자 코드북, padding 을 채울때 원문의 크기를 그대로 패딩으로 채움 
  - `GMAC` : `GHASH`로 구성된 MAC,
- `Nonce`: [Nonce Reference](./Nonce.md).  