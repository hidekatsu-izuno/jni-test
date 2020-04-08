public class ZipFile {
    static {
        System.loadLibrary("jni_test");
    }

    private static native long open(byte[] filename);
    private static native void entries(long file, EntryConsumer func);
    private static native void close(long file);

    public static void main(String[] args) {
        long file = open("./tests/LICENSE.zip".getBytes());
        try {
            entries(file, (index, filename) -> {
                System.out.println("" + index + ", " + filename);
            });
        } finally {
            close(file);
        }

    }

    private static interface EntryConsumer {
        public void accept(int index, byte[] filename);
    }
}