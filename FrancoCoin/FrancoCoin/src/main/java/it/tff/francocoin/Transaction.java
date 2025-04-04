//Properties
//Hash(String)
//Quantity
//Fee
//Sender (Address)
//Receiver (Address)
//CreationDate (Date Time)
//SenderPublicKey (String)
//SenderSignature(String)


package it.tff.francocoin;

import org.bouncycastle.crypto.digests.Blake2sDigest;
import org.bouncycastle.util.encoders.Hex;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.time.LocalDateTime;

import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.HashMap;

public class Transaction {
  private String hash;
  private BigInteger quantity;
  private BigDecimal fee;
  private Address sender;
  private Address receiver;
  private LocalDateTime creationDate;
  private String senderPublicKey;
  private String senderSignature;

  public Transaction() {}

  public Transaction(BigInteger quantity, BigDecimal fee, Address sender, 
      Address receiver, LocalDateTime creationDate, String senderPublicKey, 
      String senderSignature) 
  {
        
        this.quantity = quantity;
        this.fee = fee;
        this.sender = sender;
        this.receiver = receiver;
        this.creationDate = creationDate;
        this.senderPublicKey = senderPublicKey;
        this.senderSignature = senderSignature;
        this.hash = computeHash();
    }
  public String getHash() {
        return hash;
    }

    public void setHash(String hash) {
        this.hash = hash;
    }

    public BigInteger getQuantity() {
        return quantity;
    }

    public void setQuantity(BigInteger quantity) {
        this.quantity = quantity;
    }

    public BigDecimal getFee() {
        return fee;
    }

    public void setFee(BigDecimal fee) {
        this.fee = fee;
    }

    public Address getSender() {
        return sender;
    }

    public void setSender(Address sender) {
        this.sender = sender;
    }

    public Address getReceiver() {
        return receiver;
    }

    public void setReceiver(Address receiver) {
        this.receiver = receiver;
    }

    public LocalDateTime getCreationDate() {
        return creationDate;
    }

    public void setCreationDate(LocalDateTime creationDate) {
        this.creationDate = creationDate;
    }

    public String getSenderPublicKey() {
        return senderPublicKey;
    }

    public void setSenderPublicKey(String senderPublicKey) {
        this.senderPublicKey = senderPublicKey;
    }

    public String getSenderSignature() {
        return senderSignature;
    }

    public void setSenderSignature(String senderSignature) {
        this.senderSignature = senderSignature;
    }
  
    //We will now concatenate the parts of the transaction to create the hash 
    //We will once again use Blake2s
    public String computeHash() {
       String data = "";
        data += (quantity != null ? quantity.toString() : "");
        data += (fee != null ? fee.toString() : "");
        data += (sender != null ? sender.getAddress() : "");
        data += (receiver != null ? receiver.getAddress() : "");
        data += (creationDate != null ? creationDate.toString() : "");
        data += (senderPublicKey != null ? senderPublicKey : "");
        data += (senderSignature != null ? senderSignature : "");

        byte[] dataBytes = data.getBytes(StandardCharsets.UTF_8);

        Blake2sDigest digest = new Blake2sDigest(256);
        digest.update(dataBytes, 0, dataBytes.length);

         byte[] hashBytes = new byte[digest.getDigestSize()];
        digest.doFinal(hashBytes, 0);

        return Hex.toHexString(hashBytes);
    }






}
