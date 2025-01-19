class DockPack < Formula
    version 'v0.1.12'
    desc "Recursively search directories for a regex pattern."
    homepage "https://github.com/MaxwellFlitton/dockpack"
  
    if OS.mac?
        url "https://github.com/Maxwellflitton/dockpack/releases/download/#{version}/dockpack-#{version}-x86_64-apple-darwin.tar.gz"
        # Presumably this by the CI/CD on the release - ignore currently
        sha256 "no_check"
    elsif OS.linux?
        url "https://github.com/MaxwellFlitton/dockpack/releases/download/#{version}/dockpack-#{version}-x86_64-unknown-linux-musl.tar.gz"
        # Presumably this by the CI/CD on the release
        sha256 "no_check"
    end
  
    conflicts_with "dockpack"
  
    def install
      bin.install "dockpack"
      man1.install "doc/rg.1"
  
      bash_completion.install "complete/dockpack.bash"
      fish_completion.install "complete/dockpack.fish"
      zsh_completion.install "complete/_dockpack"
    end
  end