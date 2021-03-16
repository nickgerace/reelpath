class Reelpath < Formula
  desc "CLI tool that prints the absolute path for a given file or directory."
  homepage "https://nickgerace.dev"
  url "https://github.com/nickgerace/reelpath/archive/1.0.1.tar.gz"
  sha256 "a7893da084de58a5ac3d53aaad1d7677a12717b1ac27fe4bba33ab8ca0b1c921"
  license "Apache-2.0"

  depends_on "rust" => :build

  uses_from_macos "zlib"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system bin/"reelpath"
  end
end
