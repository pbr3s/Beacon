<h1 align="left">Beacon Project</h1>

###

<p align="left">Beacon é uma ideia projetada para identificar e mapear redes de distribuição de conteúdo abusivo em plataformas de imagens (DeviantArt, Pinterest, etc.).<br><br>Diferente de abordagens tradicionais que dependem de reconhecimento de imagem, beacon foca no comportamento da rede e metadados contextuais.</p>

###

<h2 align="left">O buraco de coelho</h2>

###

<p align="left">Algoritmos de recomendação são agnósticos ao conteúdo moral. eles buscam similaridade estatística. Beacon inverte esse lógica: utilizamos o próprio algoritmo das plataformas para rastrear e documentar criminosos.</p>

###

<h2 align="left">Arquitetura</h2>

###

Para manter o projeto funcional, dividiremos em 2 partes. 
- Orquestrador ( esse projeto ): Tem como objetivo fazer todo o trabalho de busca e documentação.
- Banco de assinaturas: As regras especificas para detecção. 
    - esse será mantido fora do código principal de modo que as técninas não sejam usadas inversamente para camuflar comportamento.

###
