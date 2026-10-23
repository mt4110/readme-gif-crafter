class Rgc < Formula
  desc "A CLI tool to craft optimized GIF demos for your READMEs from MP4 recordings"
  homepage "https://github.com/mt4110/readme-gif-crafter"
  url "https://github.com/mt4110/readme-gif-crafter/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "e88f85650a1d9fb1de7ca64e060a8deb710aae4e7c3c7bdb64c8f65431218c31"
  license "MIT"

  depends_on "rust" => :build
  depends_on "ffmpeg"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/rgc", "--help"
  end
end
