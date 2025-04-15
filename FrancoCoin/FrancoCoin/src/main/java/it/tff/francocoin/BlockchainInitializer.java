package it.tff.francocoin;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Component;

import javax.annotation.PostConstruct;
import java.io.*;
import java.math.BigInteger;
import java.nio.file.Files;
import java.security.SecureRandom;
import java.sql.Connection;
import java.sql.SQLException;
import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.HashMap;

// driver per SQLITE -> 


@Component
public class BlockchainInitializer {

    @Autowired
    private JdbcTemplate jdbcTemplate;

    private Address walletAddress;
    private Integer latestBlockHeight;

    @PostConstruct
    public void init() {

        File walletFile = new File("wallet.dat");
        if (!walletFile.exists()) {
            System.out.println("Wallet.dat not found");

            SecureRandom random = new SecureRandom();
            BigInteger privateKey = new BigInteger(256, random);

            walletAddress = Address.fromPrivateKey(privateKey);

            try (PrintWriter writer = new PrintWriter(walletFile)) {
                // Save the private key in the wallet.dat file
                writer.print(privateKey.toString());
            } catch (IOException e) {
                System.err.println("Error while saving wallet.dat");
                System.exit(1);
            }
        } else {
            // Load wallet.dat
            System.out.println("Wallet.dat found");
            try {
                String privateKeyStr = Files.readString(walletFile.toPath());
                BigInteger privateKey = new BigInteger(privateKeyStr.trim());
                walletAddress = Address.fromPrivateKey(privateKey);
            } catch (IOException e) {
                System.err.println("Error reading wallet.dat: " + e.getMessage());
                System.exit(1);
            }

            System.out.println("Address loaded from wallet.dat: " + walletAddress.getAddress());

        }

        // Now we check if the blocks table exists and if necessary we create a new one
        // Using the injected jdbcTemplate ensures we're using the Spring-configured datasource
        try {
            String tableCheckQuery = "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='blocks'";
            int count = jdbcTemplate.queryForObject(tableCheckQuery, Integer.class);
            if (count == 0) {
                System.out.println("Tabella 'blocks' non esistente. Creazione della tabella...");
                jdbcTemplate.execute("CREATE TABLE blocks (" +
                        "height INT PRIMARY KEY, " +
                        "previousBlock VARCHAR(1024), " +
                        "transactions VARCHAR(4096), " +
                        "hash VARCHAR(1024)" +
                        ")");
            }
        } catch (Exception e) {
            System.err.println("Error in the  check/creation of the blocks table: " + e.getMessage());
            System.exit(1);
        }
        try {
            Integer maxHeight = jdbcTemplate.queryForObject("SELECT MAX(height) FROM blocks", Integer.class);
            if (maxHeight == null) {
                System.out.println("No block found. Creating block 0 (genesis)...");
                // Creo il blocco genesis: altezza 0, nessun blocco precedente e nessuna transazione (array vuoto)
                Block genesisBlock = Block.create(0, null, new Transaction[0]);
                // Salvo il blocco nel database
                jdbcTemplate.update("INSERT INTO blocks (height, previousBlock, transactions, hash) VALUES (?, ?, ?, ?)",
                        genesisBlock.getHeight(),
                        (genesisBlock.getPreviousBlock() != null ? genesisBlock.getPreviousBlock().toString() : ""),
                        (genesisBlock.getTransactions() != null ? Arrays.toString(genesisBlock.getTransactions()) : ""),
                        genesisBlock.getHash());
                this.latestBlockHeight = 0; // Set height to 0 for genesis block
            } else {
                System.out.println("Blocks found. Maximum height: " + maxHeight + ". Startin the HTTP server...");
                this.latestBlockHeight = maxHeight; // Store the max height
            }
        } catch (Exception e) {
            System.err.println("Error in the selection of the blocks' heights: " + e.getMessage());
            System.exit(1);
        }

        System.out.println("Blockchain inizialized with wallet: " + (walletAddress != null ? walletAddress.getAddress() : "N/A"));

    }

    // Add Getters
    public Address getWalletAddress() {
        return walletAddress;
    }

    public Integer getLatestBlockHeight() {
        return latestBlockHeight;
    }
}

