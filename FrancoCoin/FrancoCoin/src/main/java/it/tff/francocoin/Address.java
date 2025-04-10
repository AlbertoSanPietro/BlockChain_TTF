package it.tff.francocoin;

import org.bouncycastle.util.encoders.Hex;
import org.web3j.crypto.*;
import java.math.BigInteger;
import org.bouncycastle.crypto.digests.Blake2sDigest;


public class Address extends AbstractAddress {
 private String address;

  public Address() {}

   public Address(String address) {
        this.address = address;
    }

     public static Address fromString(String addressString) {
        return new Address(addressString);
    }


public static Address fromPrivateKey(BigInteger privateKey) {
        byte[] privateKeyBytes = privateKey.toByteArray();
        
        Blake2sDigest digest = new Blake2sDigest(256);
        digest.update(privateKeyBytes, 0, privateKeyBytes.length);
        
        byte[] hash = new byte[digest.getDigestSize()];
        digest.doFinal(hash, 0);
        
        String derivedAddress = Hex.toHexString(hash);
        return new Address(derivedAddress);
    }
   
   public BigInteger getBalance() {
        return BigInteger.ZERO;
    }

    public String getAddress() {
        return address;
    }

    public void setAddress(String address) {
        this.address = address;
    }

}



