<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    exclude-result-prefixes="xs"
    version="3.0">
    <!-- xsl:merge test on city and weather data. -->
    <!-- the data files cities-SE.xml and weather-SE.xml were downloaded from openweathermap.org. The data
          was originally in JSON format; it was converted to XML, filtered to extract the data for Sweden,
          and sorted by city name in Swedish collating order. -->
    <xsl:output method="xml" indent="no"/>

    
    <xsl:template name="xsl:initial-template">
        <weather>
            <xsl:merge>                
                <xsl:merge-source name="cities"
                        streamable="yes"
                		for-each-source="'cities-SE.xml'"
                		select="city-list/city">
                    <xsl:merge-key select="name" collation="http://www.w3.org/2013/collation/UCA?lang=de;caseFirst=upper;alternate=shifted"/>
                </xsl:merge-source>
                <xsl:merge-source name="weather"
                        streamable="yes"
                		for-each-source="'weather-SE.xml'"
                		select="city-list/record">
                    <xsl:merge-key select="(.//sv, city/name)[1]" collation="http://www.w3.org/2013/collation/UCA?lang=de;caseFirst=upper"/>
                </xsl:merge-source>
                <xsl:merge-action>
                  <xsl:variable name="g" select="current-merge-group()"/>
                  <xsl:if test="$g/self::city and $g/self::record">
                    <city name="{current-merge-key()}" lon="{$g/self::city/coord/lon}" lat="{$g/self::city/coord/lat}" 
                          temp="{$g/self::record/main/temp}" wind="{$g/self::record/wind/speed}"/>
                  </xsl:if>        
                </xsl:merge-action>
            </xsl:merge>            
        </weather>
    </xsl:template>
</xsl:stylesheet>