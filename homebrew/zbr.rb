class Zbr < Formula
  desc "ZBR CLI, A simple and powerful Discord bot scripting engine."
  homepage "https://zbrlang.tech"
  version "1.8.3"

  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/zbrlang/zbr/releases/latest/download/zbr-darwin-x64"
    sha256 "REPLACE_ME_SHA256_DARWIN_X64"
  elsif OS.mac? && Hardware::CPU.arm?
    url "https://github.com/zbrlang/zbr/releases/latest/download/zbr-darwin-arm64"
    sha256 "REPLACE_ME_SHA256_DARWIN_ARM64"
  elsif OS.linux? && Hardware::CPU.intel?
    url "https://github.com/zbrlang/zbr/releases/latest/download/zbr-linux-x64"
    sha256 "REPLACE_ME_SHA256_LINUX_X64"
  elsif OS.linux? && Hardware::CPU.arm?
    url "https://github.com/zbrlang/zbr/releases/latest/download/zbr-linux-arm64"
    sha256 "REPLACE_ME_SHA256_LINUX_ARM64"
  end

  def install
    if OS.mac? && Hardware::CPU.intel?
      bin.install "zbr-darwin-x64"
      mv bin/"zbr-darwin-x64", bin/"zbr"
    elsif OS.mac? && Hardware::CPU.arm?
      bin.install "zbr-darwin-arm64"
      mv bin/"zbr-darwin-arm64", bin/"zbr"
    elsif OS.linux? && Hardware::CPU.intel?
      bin.install "zbr-linux-x64"
      mv bin/"zbr-linux-x64", bin/"zbr"
    elsif OS.linux? && Hardware::CPU.arm?
      bin.install "zbr-linux-arm64"
      mv bin/"zbr-linux-arm64", bin/"zbr"
    end
  end

  test do
    system "#{bin}/zbr", "--version"
  end
end
