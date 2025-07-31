<?xml version="1.0"?> 

<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" version="3.0"
xmlns:err="http://www.w3.org/2005/xqt-errors"
exclude-result-prefixes=" err"
>

<!-- Variables declared in xsl:try not visible in xsl:catch -->
<!-- Expect error "undeclared variable -->

<xsl:output indent="yes"/>


<xsl:template name="main">
<out>
  <xsl:try>
    <xsl:variable name="pi" select="3.14159"/>
    <xsl:attribute name="test" select="1 div 0"/>
    <xsl:catch errors="err:FOAR9876 err:FOAR0001">
      <xsl:attribute name="test" select="$pi"/>
    </xsl:catch>
    <xsl:catch errors="*" select="'Unexpected error'"/>
  </xsl:try>
</out>  
</xsl:template>

</xsl:stylesheet>
