<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <!-- basic tests with xsl:on-empty error, multiple -->
  
    <xsl:template name="xsl:initial-template">
        <empty>            
            <xsl:on-empty select="()"/> 
            <xsl:on-empty select="42"/> 
        </empty>
    </xsl:template>

</xsl:stylesheet>
