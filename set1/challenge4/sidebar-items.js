initSidebarItems({"fn":[["get_message","This function is used when the process_file function has found a line with at least 3 words. If a line contains two words, it may be \"UTF8-8 Error\" and we don't want that line or message. The same loop is run again for 0 to 255 testing each single char key with the XOR operation and it prints the best matching plain-text message to the console (using -- --nocapture)"],["process_file","This function reads the data file line by line and then sends the line to the wc_single_char_xor_encrypted_line function and  if line produces more than 2 words using one of the single characters the secret message is extracted and printed to the screen."],["wc_single_char_xor_encrypted_line","This finds and returns the highest word count it can find decrypting a hex string using XOR and some single character key between 0 and 255 decimal. The hex string relating to the highest word count could be used with get_message to see the de-crypted message and determine if it is human-readable."]]});