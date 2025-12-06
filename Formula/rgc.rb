class Rgc < Formula
  desc "A CLI tool to craft optimized GIF demos for your READMEs from MP4 recordings"
  homepage "https://github.com/masakitakemura/readme-gif-crafter"
  url "https://github.com/masakitakemura/readme-gif-crafter/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000" # TODO: Update with actual SHA256 of the release tarball
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
