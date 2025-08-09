# Ssec

Ssec é um programa de linha de comando (CLI) para realizar montagens e desmontagem de volumes do
[VeraCrypt](https://veracrypt.eu/en/Home.html) de forma automática passando a senha no Windows. Também tem a opção de adicionar
serviços após a montagem e antes da desmontagem.
Ssec não resolve volumes que usam `passfile` (senha por arquivo) do VeraCrypt.

> IMPORTANTE: Todos os volumes têm que ter a mesma senha caso você tenha mais de um volume na lista
> para montar.

## Dependências

* [VeraCrypt](https://veracrypt.eu/en/Home.html) (Windows)
* [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/pt-br/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022) (Windows)

## Configuração

**1** - Após instalar todas as dependências acima, crie uma pasta em local de sua preferência e
dentro da mesma crie um arquivo com nome de `config.json` com essa estrutura abaixo:

Exemplo de `config.json`:

```json
{
  "veracrypt": {
    "path": "C:\\Program Files\\VeraCrypt\\VeraCrypt.exe"
  },
  "ssec": {
    "volumes": [
      ["\\Device\\Harddisk1\\Partition2", "D"],
      ["D:\\volume02.hc", "B"]
    ]
  },
    "commands": {
    "mount": {
      "enable": true,
      "services": [
        "NGinx"
      ]
    },
    "umount": {
      "enable": true,
      "services": [
        "NGinx"
      ]
    }
  }
}
```

Em `"veracrypt": {"path": ""}`, você deve colocar o caminho absoluto do binário do VerCrypt.

Em `"ssec": {"volumes": [["",""],["",""],["",""]]}`, você deve colocar o caminho absoluto do(s)
volume(s) que você queira montar, e a letra de montagem do volume. Você pode adicionar quantos
volumes quiser.

Em `"commands": {"mount": "enable":}` e `"commands": {"umount": "enable":}`, você deve colocar
`true` ou `false`. Se for `true`, o **Ssec** irá executar os comandos especificados, caso seja
`false` não será executado comando nenhum.

Em `"commands": {"mount": "services": ["", "", ""]}`, o **Ssec**, irá executar comandos APÓS os
volumes estiverem montados.

Em `"commands": {"umount": "services": ["", "", ""]}`,o **Ssec**, irá executar comandos ANTES os
volumes serem desmontados.

> NOTA: Caso você use `true` para `"commands": {"mount": "enable":}` ou
> `"commands": {"umount": "enable":}`, e os comandos requer elevação de ADMINISTRADOR,
> o **Ssec** terá que ser executado como administrador no Windows.

## Instalação

**1** - Baixe a última versão do **Ssec** [aqui](https://github.com/williamcanin/ssec/tags)

**2** - Adiciona nas variáveis de ambiente do seu sistema operacional, o caminho do binário do **Ssec**.

**3** - Abra o CMD no Windows, e execute `ssec help`.
Se aparecer o menu de ajuda do **Ssec** então a instalação foi concluída.


---
(c) 2025 - William C. Canin
