package org.github.buruma;

import org.apache.curator.test.TestingServer;

/**
 * Hello world!
 */
public class App {
    public static void main(String[] args) throws Exception {
        TestingServer testingServer = null;
        try {
            testingServer = new TestingServer(2181);
        } catch (Exception e) {
            e.printStackTrace();
            if (testingServer != null) {
                testingServer.close();
            }
        }
    }
}
