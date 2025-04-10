package it.tff.francocoin;

import org.bouncycastle.util.encoders.Hex;
import org.web3j.crypto.*;
import java.math.BigInteger;


public abstract class AbstractAddress {

  public abstract BigInteger getBalance();
}




