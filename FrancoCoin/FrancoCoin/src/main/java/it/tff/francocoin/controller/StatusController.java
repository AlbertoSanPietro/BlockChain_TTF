package it.tff.francocoin.controller;

import it.tff.francocoin.Address;
import it.tff.francocoin.BlockchainInitializer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.HashMap;
import java.util.Map;

@RestController
public class StatusController {

    @Autowired
    private BlockchainInitializer blockchainInitializer;

    @GetMapping("/")
    public Map<String, Object> getStatus() {
        Map<String, Object> status = new HashMap<>();
        Address wallet = blockchainInitializer.getWalletAddress();

        if (wallet != null) {
            status.put("walletAddress", wallet.getAddress());
            status.put("publicKey", wallet.getPublicKey());
        } else {
            status.put("walletAddress", "N/A");
            status.put("publicKey", "N/A");
        }

        status.put("latestBlockHeight", blockchainInitializer.getLatestBlockHeight() != null ? blockchainInitializer.getLatestBlockHeight() : "N/A");

        return status;
    }
}
