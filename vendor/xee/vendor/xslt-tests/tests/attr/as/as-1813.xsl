<?xml version="1.0" encoding="UTF-8"?>
<t:transform xmlns:my="http://www.example.com/ns/various"
   xmlns:t="http://www.w3.org/1999/XSL/Transform" exclude-result-prefixes="my" version="2.0">
   <!-- Purpose: Test of xsl:template which contains an attribute node from input file, or explicitly 
  				created xsl:attribute and  @as="attribute (QName, user-derived-simple-type)".Types 
  				tested:  - user-defined atomic, list, union-->

   <t:import-schema namespace="http://www.example.com/ns/various"
      schema-location="variousTypesSchemaAs.xsd"/>

   <t:template match="/">
      <out>
         <temp1>
            <t:call-template name="temp1"/>
         </temp1>
         <temp2>
            <t:call-template name="temp2"/>
         </temp2>
         <temp3>
            <t:call-template name="temp3"/>
         </temp3>
      </out>
   </t:template>

   <t:template name="temp1" as="attribute(my:specialPart, my:partNumberType)">
      <t:copy-of select="my:userNode/@my:specialPart" validation="strict"/>
   </t:template>

   <t:template name="temp2" as="attribute(my:listParts, my:myListType)">
      <t:copy-of select="my:userNode/@my:listParts" validation="strict"/>
   </t:template>

   <t:template name="temp3" as="attribute(my:union, my:partIntegerUnion)">
      <t:attribute name="my:union" type="my:partIntegerUnion">11111</t:attribute>
   </t:template>
</t:transform>
