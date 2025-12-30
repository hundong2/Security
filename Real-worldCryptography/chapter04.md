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

# 올인원 구조 인증 

## AEAD 

- 연관 데이터 인증 암호화 ( Authenticated encryption with associated data): 일체형 구조 
- [Example code ](./AEAD/hello.js). 

## AES-GCM AEAD 

- 갈루아/카운터 모드(Galois/Counter Mode)가 있는 AES
- AES 에 대한 하드웨어 자원을 활용하여, 고성능을 위해 이를 효율적으로 구현할 수 있는 MAC(GMAC)을 사용하여 설계됨. 
- [AES-GCM](./AES_GCM.md). 