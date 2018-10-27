unsigned char allocate(unsigned long len) {
  unsigned char * mem = malloc(len);
  
  for (unsigned long i=0;i<len;i++) {
    mem[i] = i%256;
  }
  
  return &mem[0];
}
