package services

import "C"

import "unsafe"

func HashPassword(password string) string {

	cpass := C.CString(password)

	hash := C.hash_password(cpass)

	goHash := C.GoString(hash)

	C.free_string(hash)
	C.free(unsafe.Pointer(cpass))

	return goHash
}

func VerifyPassword(password string, hash string) bool {

	cpass := C.CString(password)
	chash := C.CString(hash)

	ok := C.verify_password(cpass, chash)

	C.free(unsafe.Pointer(cpass))
	C.free(unsafe.Pointer(chash))

	return ok == 1
}
