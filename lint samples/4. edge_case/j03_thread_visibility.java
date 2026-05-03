public class Flag {
    private boolean running = true;

    public void stop() { running = false; }

    public void run() {
        while (running) {}
    }
}
