<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" version="3.0">

<xsl:template name="main">
  <out>
  <xsl:source-document streamable="true" href="snapshot-02.xml">
     <xsl:for-each select="snapshot(works/department/employee)">
        <emp name="{@name}" dept="{../@name}" nr="{empnum}" hours="{hours}"/>
     </xsl:for-each>
  </xsl:source-document>
  </out>
</xsl:template>


</xsl:stylesheet>