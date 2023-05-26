public class Hmm {
    public void hmm() {
        if (Thread.currentThread().getContextClassLoader() == null) {
            throw new java.lang.RuntimeException("hmm");
        }
    }
}