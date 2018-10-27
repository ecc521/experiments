unsigned char allocate(unsigned long len) {
  unsigned char * mem = malloc(len);
  return &mem;
}


unsigned char set(unsigned char * ptr, unsigned long len) {
  for (unsigned long i=0;i<len;i++) {
    *(ptr+i) = i%256;
  }
  return ptr;
}
