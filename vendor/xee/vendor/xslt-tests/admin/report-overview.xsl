<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    xmlns:math="http://www.w3.org/2005/xpath-functions/math"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:xt="http://xt3report.com/functions"
    xmlns:cat="http://www.w3.org/2012/10/xslt-test-catalog"
    xmlns:f="urn:functions"
    xpath-default-namespace="http://www.w3.org/2012/10/xslt-test-catalog"
    exclude-result-prefixes="math xs xsi cat xt"
    expand-text="yes"
    version="3.0">
    
    <xsl:output indent="yes" method="html" html-version="5.0"/>

    <xsl:strip-space elements="*"/>
    
    <xsl:attribute-set name="cell">
    </xsl:attribute-set>
    
    <xsl:attribute-set name="first-cell">
    </xsl:attribute-set>
    
    <xsl:attribute-set name="last-cell">
    </xsl:attribute-set>

    <xsl:attribute-set name="header-cell">
        <xsl:attribute name="class">header</xsl:attribute>
    </xsl:attribute-set>
    
    <xsl:attribute-set name="last-header-cell" use-attribute-sets="header-cell">
    </xsl:attribute-set>
    
    <xsl:template match="/">
        <html>
            <head>
                <title>XSLT 3.0 test case overview</title>
                <style type="text/css" xsl:expand-text="no">
                    * {
                        font-family: Arial, sans-serif;
                    }
                    
                    table {
                        border: 1pt gray solid;
                        width: 600px;
                    }
                    
                    table * {
                        font-size: 10pt;
                    }
                    
                    th {
                        border: 0pt gray solid;
                        padding: 3pt .8em 2pt .8em;
                        text-align: left;
                    }
                    
                    table th.header {
                        font-size: 12pt;
                    }
                    
                    tr > th:first-child {
                        padding-left: 4pt;
                    }
                    
                    td {
                        border: 0pt gray solid;
                        padding: 2pt .8em 2pt .8em;
                        text-align: right;
                    }

                    caption {
                        font-weight: bold;
                        font-size: 14pt;
                        padding: 3em 0 .6em 0;
                    }
                </style>
            </head>
            <body>
                <h1>XSLT 3.0 test case overview</h1>
                <table style="border-style:collapse;border:1pt grey solid;border-spacing: 0;">
                    <caption>Overview by XT3 category</caption>
                    <xsl:apply-templates select="report/overview"/>
                </table>
                <xsl:apply-templates select="report/changes-since-xslt2"/>
                <xsl:apply-templates select="report/category" />
                <xsl:apply-templates select="report/category/test-set/keywords" />
            </body>
        </html>
    </xsl:template>
    
    <xsl:template match="report/overview">
        <tr>
            <th xsl:use-attribute-sets="header-cell">&#xA;</th>
            <th xsl:use-attribute-sets="header-cell">XSLT 2.0</th>
            <th xsl:use-attribute-sets="header-cell">XSLT 3.0</th>
            <th xsl:use-attribute-sets="header-cell">Unclassified / other</th>
            <th xsl:use-attribute-sets="last-header-cell">Total</th>
        </tr>
        <tr>
            <th xsl:use-attribute-sets="first-cell">All categories</th>
            <td xsl:use-attribute-sets="cell">{xslt2-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt3-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt-other/@count}</td>
            <td xsl:use-attribute-sets="last-cell">{@total-tests-in-xt3}</td>
        </tr>
        <xsl:apply-templates select="../category/overview" />
    </xsl:template>
    
    <xsl:template match="category/overview">
        <tr>
            <th xsl:use-attribute-sets="first-cell">
                <a href="#{f:encode(parent::category/@name)}-xt3"  id="{f:encode(parent::category/@name)}-xt3-source">{parent::category/@name}</a>
            </th>
            <td xsl:use-attribute-sets="cell">{xslt2-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt3-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt-other/@count}</td>
            <td xsl:use-attribute-sets="last-cell">{@total-tests-in-category}</td>
        </tr>
    </xsl:template>
    
    <xsl:template match="report/category">
        <table>
            <caption><a id="{f:encode(@name)}-xt3" />{@name} (XT3) <a style="text-decoration:none" href="#{f:encode(@name)}-xt3-source" >↑↑</a></caption>
            <xsl:apply-templates select="overview" mode="cat-overview" />
            <xsl:apply-templates select="test-set" />
        </table>
    </xsl:template>

    <xsl:template match="category/overview" mode="cat-overview">
        <tr>
            <th xsl:use-attribute-sets="first-cell">{parent::category/@name} (all)</th>
            <td xsl:use-attribute-sets="cell">{xslt2-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt3-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt-other/@count}</td>
            <td xsl:use-attribute-sets="last-cell">{@total-tests-in-category}</td>
        </tr>
    </xsl:template>

    <xsl:template match="category/test-set">
        <tr>
            <th xsl:use-attribute-sets="first-cell">
                <a href="#{f:encode(@description)}-keywords" id="{f:encode(@description)}-key-overview">{@description}</a>
            </th>
            <td xsl:use-attribute-sets="cell">{xslt2-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt3-specific/@count}</td>
            <td xsl:use-attribute-sets="cell">{xslt-other/@count}</td>
            <td xsl:use-attribute-sets="last-cell">{@total-tests}</td>
        </tr>
    </xsl:template>
    

    <xsl:template match="changes-since-xslt2">
        <table style="border-style:collapse;border:1pt grey solid;border-spacing: 0;">
            <caption>Overview by 3.0 feature</caption>
    
            <tr>
                <th xsl:use-attribute-sets="header-cell">&#xA;</th>
                <th xsl:use-attribute-sets="header-cell">Total</th>
            </tr>
            <tr>
                <th xsl:use-attribute-sets="first-cell">All features</th>
                <td xsl:use-attribute-sets="cell">{sum(feature/@count[not(contains(., 'unknown'))])}</td>
            </tr>
            <xsl:for-each-group select="feature" group-adjacent="@group">
                <tr>
                    <th xsl:use-attribute-sets="first-cell">
                        <a href="#{f:encode(current-grouping-key())}" id="{f:encode(current-grouping-key())}-feature-source">{current-grouping-key()}</a>
                    </th>
                    <td xsl:use-attribute-sets="cell">{sum(current-group()/self::feature/@count[not(contains(., 'unknown'))])}</td>
                </tr>    
            </xsl:for-each-group>
        </table>
        
        <xsl:for-each-group select="feature" group-adjacent="@group">
            <table>
                <caption><a id="{f:encode(current-grouping-key())}"/>Feature: {current-grouping-key()} <a  href="#{f:encode(current-grouping-key())}-feature-source">↑↑</a></caption>
                <xsl:apply-templates select="current-group()" />
            </table>    
        </xsl:for-each-group>
    </xsl:template>
    
    <xsl:template match="feature">
        <tr>
            <th xsl:use-attribute-sets="first-cell">{@name}</th>
            <td xsl:use-attribute-sets="cell">{@count}</td>
        </tr>
    </xsl:template>
    
    <xsl:template match="keywords">
        <table style="width:300px">
            <caption><a id="{f:encode(../@description)}-keywords" />{../@name} (keywords) <a style="text-decoration:none" href="#{f:encode(../@description)}-key-overview" >↑↑</a>
                <div style="padding-top:.5em;font-size:8pt;font-weight:normal">(description: {../@description})</div>
            </caption>
            <xsl:if test="key">
                <tr>
                    <th>Key</th>
                    <th>Count</th>
                </tr>
                <xsl:apply-templates select="key" />
            </xsl:if>
            <xsl:if test="not(key)">
                <tr>
                    <td colspan="2">No keywords specified for this category yet</td>
                </tr>
            </xsl:if>
        </table>
        
    </xsl:template>
    
    <xsl:template match="key">
        <tr>
            <th xsl:use-attribute-sets="first-cell">{@name}</th>
            <td xsl:use-attribute-sets="cell">{@count}</td>
        </tr>
    </xsl:template>
    
    <xsl:function name="f:encode">
        <xsl:param name="id" />
        <xsl:value-of select="translate($id, ' ', '-')" />
    </xsl:function>
    
    
</xsl:stylesheet>