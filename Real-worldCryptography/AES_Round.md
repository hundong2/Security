# AES 라운드 변환(SubBytes, ShiftRows, MixColumns, AddRoundKey)
AES 한 라운드는 바이트 치환·행 이동·열 혼합·라운드키 XOR의 네 단계로 구성되며, 혼돈(confusion)과 확산(diffusion)을 달성합니다.
## 예시 파일
[FIPS 197: Advanced Encryption Standard (AES) — 공식 표준 PDF](https://csrc.nist.gov/publications/detail/fips/197/final)
## 답변
- SubBytes
  - 각 바이트를 Rijndael S-box로 치환하는 비선형 단계입니다.
  - S-box는 GF(2^8)에서의 역원 계산 후 아핀 변환으로 만들어져, 차분/선형 해석에 강합니다.
  - 효과: 혼돈(confusion) 제공, 바이트 값과 키의 관계를 비선형화.

- ShiftRows
  - 상태(state) 4×4 바이트 행렬의 각 행을 좌측으로 순환 이동합니다.
    - 0번째 행: 0칸, 1번째: 1칸, 2번째: 2칸, 3번째: 3칸 이동.
  - 효과: 열 간 결합을 유도해 SubBytes로 만들어진 비선형성을 전체 블록에 확산.

- MixColumns
  - 각 열을 GF(2^8)에서 고정 행렬과 곱해 혼합합니다(행렬 [2 3 1 1; 1 2 3 1; 1 1 2 3; 3 1 1 2]).
  - 1바이트 변화가 열의 4바이트 모두에 퍼지도록 만드는 확산 단계입니다.
  - 마지막 라운드에서는 MixColumns를 수행하지 않습니다.

- AddRoundKey
  - 라운드 키(키 스케줄로 생성)를 상태와 XOR합니다.
  - 효과: 비밀을 주입하는 유일한 단계로, 다른 단계의 혼돈/확산과 결합해 전체 안전성을 형성.

라운드 구조(요약):
- 초기: AddRoundKey
- 중간 라운드(예: AES-128은 9회): SubBytes → ShiftRows → MixColumns → AddRoundKey
- 마지막 라운드: SubBytes → ShiftRows → AddRoundKey

왜 이렇게 조합하나?
- SubBytes(비선형) + ShiftRows/MixColumns(확산) + AddRoundKey(비밀 주입)가 결합되어, 작은 입력/키 변화가 전체 블록에 빠르게 퍼지며 역상·구분 공격에 강한 PRP처럼 동작하게 합니다.

### 추가 자료
- [AES 위키: 라운드 함수 개요](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard#High-level_description_of_the-algorithm)
- [Rijndael S-box 생성 원리](https://en.wikipedia.org/wiki/Rijndael_S-box)
- [MixColumns 상세 설명(수학적 배경)]https://en.wikipedia.org/wiki/Rijndael_MixColumns)