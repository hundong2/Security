# pseudorandom permutations에서 pseudo의 뜻은?

“완전히 무작위”는 아니지만, 효율적 공격자에게는 무작위처럼 구분되지 않는다는 의미입니다. 즉, 키로 선택된 순열(전단사 함수)이 진짜 무작위 순열과 계산적으로(현실적 시간 안에) 구분 불가능해야 합니다.

## 예시 파일

[Pseudorandom permutation (Wikipedia)](https://en.wikipedia.org/wiki/Pseudorandom_permutation)

## 답변

핵심 요약:

- PRP 정의: 키 k로 정의된 순열 F_k: {0,1}^n → {0,1}^n 이 “무작위 순열”처럼 보이는 함수족. 공격자는 F_k(·) 오라클을 진짜 무작위 순열 Π(·)과 구분하기가 계산적으로 어렵습니다.
- pseudo의 뜻: 정보이론적으로 완전 무작위가 아니라, “계산적으로(random-looking to any efficient adversary)” 무작위처럼 보인다는 의미입니다.
- PRF와 차이:
  - PRF: 임의 길이 도메인/범위의 일반 함수(전단사일 필요 없음).
  - PRP: 반드시 전단사(1:1, onto), 역함수 F_k^{-1}가 존재하는 고정 블록 크기 함수.
- 강한 PRP(SPRP): 정방향과 역방향 오라클(F_k, F_k^{-1})을 모두 허용해도 무작위 순열과 구분 불가해야 함.
- 예: AES는 128비트 블록에 대해 키마다 하나의 순열을 정하는 블록암호로, “AES는 (강한) PRP처럼 동작한다”는 가정 하에 사용됩니다.
- 왜 중요한가: 블록암호 안전성의 형식적 가정이며, 안전한 모드(GCM/CTR/CBC 등)를 구성할 때 기본 전제입니다.

직관 비유:
- 진짜 난수 순열은 매번 무작위로 뽑은 자물쇠 배치도.
- PRP는 “키로 선택된” 고정 배치도이지만, 외부에서 보면 무작위와 다를 바 없는 배치도.

### 추가 자료
- [Pseudorandom function family (Wikipedia)](https://en.wikipedia.org/wiki/Pseudorandom_function_family)
- [FIPS 197: AES 표준(PDF)](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf)
- [NIST SP 800-38A: 블록암호 운용 모드](https://csrc.nist.gov/publications/detail/sp/800-38a/final)