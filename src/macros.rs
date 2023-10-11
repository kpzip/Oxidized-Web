macro_rules! responsify {
    (100) => {
        "HTTP/1.1 100 CONTINUE"
    };
    (101) => {
        "HTTP/1.1 101 SWITCHING PROTOCOLS"
    };
    (102) => {
        "HTTP/1.1 102 PROCESSING"
    };
    (103) => {
        //Future Use
        "HTTP/1.1 103 EARLY HINTS"
    };
    (200) => {
        "HTTP/1.1 200 OK"
    };
    (201) => {
        "HTTP/1.1 201 CREATED"
    };
    (202) => {
        "HTTP/1.1 202 ACCEPTED"
    };
    (203) => {
        "HTTP/1.1 203 NON-AUTHORITATIVE INFORMAION"
    };
    (204) => {
        "HTTP/1.1 204 NO CONTENT"
    };
    (205) => {
        "HTTP/1.1 205 RESET CONTENT"
    };
    (206) => {
        "HTTP/1.1 206 PARTIAL CONTENT"
    };
    (207) => {
        "HTTP/1.1 207 MULTI-STATUS"
    };
    (208) => {
        "HTTP/1.1 208 ALREADY REPORTED"
    };
    (226) => {
        "HTTP/1.1 226 IM USED"
    };
    (300) => {
        "HTTP/1.1 300 MULTIPLE CHOICES"
    };
    (301) => {
        "HTTP/1.1 301 MOVED PERMANENTLY"
    };
    (302) => {
        "HTTP/1.1 302 FOUND"
    };
    (303) => {
        "HTTP/1.1 303 SEE OTHER"
    };
    (304) => {
        "HTTP/1.1 304 NOT MODIFIED"
    };
    (305) => {
        //Depricated
        "HTTP/1.1 305 USE PROXY"
    };
    (306) => {
        //Depricated
        "HTTP/1.1 306 UNUSED"
    };
    (307) => {
        "HTTP/1.1 307 TEMPORARY REDIRECT"
    };
    (308) => {
        "HTTP/1.1 308 PERMANENT REDIRECT"
    };
    (400) => {
        "HTTP/1.1 400 BAD REQUEST"
    };
    (401) => {
        "HTTP/1.1 401 UNAUTHORIZED"
    };
    (402) => {
        "HTTP/1.1 402 PAYMENT REQUIRED"
    };
    (403) => {
        "HTTP/1.1 403 FORBIDDEN"
    };
    (404) => {
        "HTTP/1.1 404 NOT FOUND"
    };
    (405) => {
        "HTTP/1.1 405 METHOD NOT ALLOWED"
    };
    (406) => {
        "HTTP/1.1 406 NOT ACCEPTABLE"
    };
    (407) => {
        "HTTP/1.1 407 PROXY AUTHENTICATION REQUIRED"
    };
    (408) => {
        "HTTP/1.1 408 REQUEST TIMEOUT"
    };
    (409) => {
        "HTTP/1.1 409 CONFLICT"
    };
    (410) => {
        "HTTP/1.1 410 GONE"
    };
    (411) => {
        "HTTP/1.1 411 LENGTH REQUIRED"
    };
    (412) => {
        "HTTP/1.1 412 PRECONDITION FAILED"
    };
    (413) => {
        "HTTP/1.1 413 PAYLOAD TOO LARGE"
    };
    (414) => {
        "HTTP/1.1 414 URI TOO LONG"
    };
    (415) => {
        "HTTP/1.1 415 UNSUPPORTED MEDIA TYPE"
    };
    (416) => {
        "HTTP/1.1 416 RANGE NOT SATISFIABLE"
    };
    (417) => {
        "HTTP/1.1 417 EXPECTATION FAILED"
    };
    (418) => {
        //???
        "HTTP/1.1 418 I'M A TEAPOT"
    };
    (421) => {
        "HTTP/1.1 421 MISDIRECTED REQUEST"
    };
    (422) => {
        "HTTP/1.1 422 UNPROCESSABLE CONTENT"
    };
    (423) => {
        "HTTP/1.1 423 LOCKED"
    };
    (424) => {
        "HTTP/1.1 424 FAILED DEPENDENCY"
    };
    (425) => {
        "HTTP/1.1 425 TOO EARLY"
    };
    (426) => {
        "HTTP/1.1 426 UPGRADE REQUIRED"
    };
    (428) => {
        "HTTP/1.1 428 PRECONDITION FAILED"
    };
    (429) => {
        "HTTP/1.1 429 TOO MANY REQUESTS"
    };
    (431) => {
        "HTTP/1.1 431 REQUEST HEADER FIELDS TOO LARGE"
    };
    (451) => {
        "HTTP/1.1 451 UNAVAILABLE FOR LEGAL REASONS"
    };
    (500) => {
        "HTTP/1.1 500 INTERNAL SERVER ERROR"
    };
    (501) => {
        "HTTP/1.1 501 NOT IMPLEMENTED"
    };
    (502) => {
        "HTTP/1.1 502 BAD GATEWAY"
    };
    (503) => {
        "HTTP/1.1 503 SERVICE UNAVAILABLE"
    };
    (504) => {
        "HTTP/1.1 504 GATEWAY TIMEOUT"
    };
    (505) => {
        "HTTP/1.1 505 HTTP VERSION NOT SUPPORTED"
    };
    (506) => {
        "HTTP/1.1 506 VARIANT ALSO NEGOTIATES"
    };
    (507) => {
        "HTTP/1.1 507 INSUFFICIENT STORAGE"
    };
    (508) => {
        "HTTP/1.1 508 LOOP DETECTED"
    };
    (510) => {
        "HTTP/1.1 510 NOT EXTENDED"
    };
    (511) => {
        "HTTP/1.1 511 NETWORK AUTHENTICATION REQUIRED"
    };
}
