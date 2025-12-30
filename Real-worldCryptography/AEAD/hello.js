// [실행 전 참고]
// - 이 스크립트는 최상위 await를 사용합니다. Node.js에서 실행하려면 다음 중 한 가지가 필요합니다.
//   1) package.json에 "type": "module" 추가, 또는
//   2) 파일 확장자를 .mjs로 사용하여 ESM 모드로 실행
// - 브라우저가 아니라 Node.js 환경에서 Web Crypto를 사용합니다(node v19+).
//   브라우저에서는 window.crypto.subtle 을, Node에서는 globalThis.crypto.subtle 을 사용합니다.

// Node v19+에서 Web Crypto는 globalThis.crypto로 제공됩니다.
const crypto = globalThis.crypto; // 필요 시: const { webcrypto: crypto } = await import('node:crypto');

// config: 생성할 대칭키의 알고리즘 정보를 담습니다.
// - name: 'AES-GCM'  → 인증된 암호(Authenticated Encryption) 모드
// - length: 128      → 키 길이(bit). AES-128/192/256 중 선택 가능
let config = { name: 'AES-GCM', length: 128 };
// keyUsages: 생성된 키로 허용할 작업 목록. encrypt/decrypt 외에 exportKey 등은 허용하지 않음.
let keyUsages = ['encrypt', 'decrypt'];
// generateKey(alg, extractable, keyUsages)
// - extractable=false → 키를 raw/JWK로 내보내(export)지 못하게 하여 보안성 향상
// - 반환값은 Promise로, 완료 시 CryptoKey를 제공합니다.
let key = await crypto.subtle.generateKey(config, false, keyUsages);

// 12바이트 IV 생성(getRandomValues는 동기)
// - GCM 모드에서는 12바이트(96비트) nonce/IV 사용이 권장됩니다.
// - 같은 키로 같은 IV를 재사용하면 심각한 보안 문제가 발생합니다(절대 재사용 금지!).
let iv = new Uint8Array(12);
crypto.getRandomValues(iv);

// TextEncoder/Decoder: 문자열을 UTF-8 바이트로, 바이트를 문자열로 변환합니다.
let te = new TextEncoder();
// AAD(Associated Data): 암호문에는 포함되지 않지만 인증되는 메타데이터. 검증 실패 시 복호화가 거부됩니다.
let ad = te.encode("some associated data");
let plaintext = te.encode("hello world");

// AES-GCM 파라미터(tagLength는 명시적으로 128비트 권장)
// - iv: 위에서 생성한 12바이트 IV
// - additionalData: AAD(선택)
// - tagLength: 인증 태그 길이(bit). 보통 128 권장(16바이트)
let param = { name: 'AES-GCM', iv, additionalData: ad, tagLength: 128 };

// encrypt/decrypt는 Promise<ArrayBuffer>를 반환합니다.
// - ciphertext: 암호문 + (환경에 따라) 태그가 결합된 바이트열
let ciphertext = await crypto.subtle.encrypt(param, key, plaintext);
let result = await crypto.subtle.decrypt(param, key, ciphertext);
// 복호 결과(ArrayBuffer)를 문자열로 디코딩해서 확인합니다.
console.log(new TextDecoder().decode(result));
console.log("Hello, JS");